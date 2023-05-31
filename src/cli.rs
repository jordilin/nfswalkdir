use clap::{arg, value_parser, Command};

#[derive(Clone, Debug)]
pub struct CliOpts {
    pub ip: String,
    pub share: String,
    pub dir: String,
    pub userid: i32,
    pub groupid: i32,
    pub numworkers: usize,
}

impl CliOpts {
    pub fn new(
        ip: impl AsRef<str>,
        share: impl AsRef<str>,
        dir: impl AsRef<str>,
        userid: i32,
        groupid: i32,
        numworkers: usize,
    ) -> CliOpts {
        CliOpts {
            ip: ip.as_ref().to_string(),
            share: share.as_ref().to_string(),
            dir: dir.as_ref().to_string(),
            userid,
            groupid,
            numworkers,
        }
    }
}

pub fn args() -> CliOpts {
    let matches = Command::new("nfs-walkdir")
        .arg(
            arg!(
                --ip <VALUE> "NFS Server IP address"
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --share <VALUE> "NFS share directory/mount path"
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --dir <VALUE> "NFS directory to traverse relative to share"
            )
            .required(true)
            .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(
                --userid <VALUE> "NFS Auth User ID"
            )
            .required(true)
            .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(
                --groupid <VALUE> "NFS Auth Group ID"
            )
            .required(true)
            .value_parser(value_parser!(i32)),
        )
        .arg(
            arg!(
                --numworkers <VALUE> "Number of worker threads"
            )
            .default_value("5")
            .value_parser(value_parser!(usize)),
        )
        .get_matches();

    let ip = matches
        .get_one::<String>("ip")
        .expect("NFS Server IP address is required");
    let share = matches
        .get_one::<String>("share")
        .expect("NFS share directory/mount path is required");
    let dir = matches
        .get_one::<String>("dir")
        .expect("NFS directory to traverse relative to share is required");
    let userid = matches
        .get_one::<i32>("userid")
        .expect("NFS Auth User ID is required");
    let groupid = matches
        .get_one::<i32>("groupid")
        .expect("NFS Auth Group ID is required");
    let numworkers = matches.get_one::<usize>("numworkers").unwrap_or(&(5_usize));
    CliOpts::new(ip, share, dir, *userid, *groupid, *numworkers)
}
