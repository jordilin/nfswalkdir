use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use libnfs::{DirEntry, EntryType, Nfs};

use crossbeam_channel::{bounded, Receiver, Sender};
use nfswalkdir::cli::args;

fn main() {
    let args = args();
    let (tx, rx): (Sender<PathBuf>, Receiver<PathBuf>) = bounded(args.numworkers);

    let (tx_proc_mtime, rx_proc_mtime): (Sender<DirEntry>, Receiver<DirEntry>) =
        bounded(args.numworkers);

    let dirs_pending_processing = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    // Start with the root directory in the NFS share directory
    tx.send(PathBuf::from(&args.dir)).unwrap();
    // increment pending dirs to process in 1 - root dir.
    let dirs_pending_processing_first = dirs_pending_processing.clone();
    *dirs_pending_processing_first.write().unwrap() += 1;
    drop(dirs_pending_processing_first);

    // Create directory scanner threads. If a directory is found inside the current
    // directory it will be sent to a worker thread for processing (Multiple
    // producer, multiple consumer pattern)
    for _ in 0..args.numworkers {
        let rx = rx.clone();
        let tx = tx.clone();
        let cli_opts = args.clone();
        let dirs_pending_processing = dirs_pending_processing.clone();
        let tx_proc_mtime = tx_proc_mtime.clone();
        handles.push(thread::spawn(move || {
            // Every thread contains a new NFS client accepting paths to be
            // processed
            let mut nfsrx = Nfs::new().unwrap();
            nfsrx.set_uid(cli_opts.userid).unwrap();
            nfsrx.set_gid(cli_opts.groupid).unwrap();
            nfsrx.set_debug(0).unwrap();
            nfsrx.mount(&cli_opts.ip, &cli_opts.share).unwrap();

            for path in rx {
                // Parent thread constantly monitors if there are any
                // directories pending processing and if not, it signals all
                // workers to stop
                if path == Path::new("STOP") {
                    drop(tx);
                    drop(tx_proc_mtime);
                    break;
                }

                // if path is dot or dotdot, no files to process
                if path == Path::new(".") || path == Path::new("..") {
                    let mut dirs_pending_processing = dirs_pending_processing.write().unwrap();
                    *dirs_pending_processing -= 1;
                    continue;
                }

                let dir = nfsrx.opendir(Path::new(&path)).unwrap();
                for entry in dir {
                    let entry = entry.unwrap();
                    if let EntryType::Directory = entry.d_type {
                        tx.send(entry.path).unwrap();
                        // increment pending dirs
                        let mut dirs_pending_processing = dirs_pending_processing.write().unwrap();
                        *dirs_pending_processing += 1;

                        continue;
                    }
                    tx_proc_mtime.send(entry).unwrap();
                }
                // finished processing current dir
                let mut dirs_pending_processing = dirs_pending_processing.write().unwrap();
                *dirs_pending_processing -= 1;
                drop(dirs_pending_processing);
            }
        }));
    }

    // Just print the file paths
    let mut handles_mtimes = vec![];
    for _ in 0..args.numworkers {
        let rx_proc_mtime = rx_proc_mtime.clone();
        handles_mtimes.push(thread::spawn(move || {
            for entry in rx_proc_mtime {
                println!("{:?}", entry.path);
            }
        }));
    }

    while *dirs_pending_processing.read().unwrap() > 0 {
        thread::sleep(std::time::Duration::from_millis(100));
    }

    // signal all workers - we are done
    for _ in 0..args.numworkers {
        tx.send(PathBuf::from("STOP")).unwrap();
    }
    drop(tx);
    drop(tx_proc_mtime);

    for handle in handles {
        handle.join().unwrap();
    }

    for handle in handles_mtimes {
        handle.join().unwrap();
    }
}
