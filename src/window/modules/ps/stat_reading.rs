#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StatReading {
    pub pid: u32,
    pub comm: String,
    pub state: char,
    pub ppid: i32,
    pub pgrp: i32,
    pub session: i32,
    pub tty_nr: i32,
    pub tpgid: i32,
    pub flags: u32,
    pub minflt: u64,
    pub cminflt: u64,
    pub majflt: u64,
    pub cmajflt: u64,
    pub utime: u64,
    pub stime: u64,
    pub cutime: i64,
    pub cstime: i64,
    pub priority: i64,
    pub nice: i64,
    pub num_threads: i64,
    pub itrealvalue: i64,
    pub starttime: u64,
    pub vsize: u64,
    pub rss: i64,
    pub rsslim: u64,
    pub startcode: u64,
    pub endcode: u64,
    pub startstack: u64,
    pub kstkesp: u64,
    pub kstkeip: u64,
    pub signal: u64,
    pub blocked: u64,
    pub sigignore: u64,
    pub sigcatch: u64,
    pub wchan: u64,
    pub nswap: u64,
    pub cnswap: u64,
    pub exit_signal: i32,
    pub processor: i32,
    pub rt_priority: u32,
    pub policy: u32,
    pub delayacct_blkio_ticks: u64,
    pub guest_time: u64,
    pub cguest_time: i64,
    pub start_data: u64,
    pub end_data: u64,
    pub start_brk: u64,
    pub arg_start: u64,
    pub arg_end: u64,
    pub env_start: u64,
    pub env_end: u64,
    pub exit_code: i32,
}

impl From<String> for StatReading {
    fn from(s: String) -> Self {
        let s = s.trim_end();

        let open = s.find('(');
        let close = s.rfind(')');

        let (pid_part, comm, rest) = match (open, close) {
            (Some(o), Some(c)) if o < c => (&s[..o], &s[o + 1..c], &s[c + 1..]),
            // No parenthesised comm: fall back to plain whitespace splitting.
            _ => {
                let mut it = s.splitn(3, char::is_whitespace);
                let pid = it.next().unwrap_or("");
                let comm = it.next().unwrap_or("");
                let rest = it.next().unwrap_or("");
                (pid, comm.trim_matches(|c| c == '(' || c == ')'), rest)
            }
        };

        // Fields 3..=52, in order.
        let mut f = rest.split_whitespace();

        fn num<T: std::str::FromStr + Default>(f: &mut std::str::SplitWhitespace) -> T {
            f.next().and_then(|v| v.parse().ok()).unwrap_or_default()
        }

        StatReading {
            pid: pid_part.trim().parse().unwrap_or_default(),
            comm: comm.to_string(),
            state: f.next().and_then(|v| v.chars().next()).unwrap_or('\0'),
            ppid: num(&mut f),
            pgrp: num(&mut f),
            session: num(&mut f),
            tty_nr: num(&mut f),
            tpgid: num(&mut f),
            flags: num(&mut f),
            minflt: num(&mut f),
            cminflt: num(&mut f),
            majflt: num(&mut f),
            cmajflt: num(&mut f),
            utime: num(&mut f),
            stime: num(&mut f),
            cutime: num(&mut f),
            cstime: num(&mut f),
            priority: num(&mut f),
            nice: num(&mut f),
            num_threads: num(&mut f),
            itrealvalue: num(&mut f),
            starttime: num(&mut f),
            vsize: num(&mut f),
            rss: num(&mut f),
            rsslim: num(&mut f),
            startcode: num(&mut f),
            endcode: num(&mut f),
            startstack: num(&mut f),
            kstkesp: num(&mut f),
            kstkeip: num(&mut f),
            signal: num(&mut f),
            blocked: num(&mut f),
            sigignore: num(&mut f),
            sigcatch: num(&mut f),
            wchan: num(&mut f),
            nswap: num(&mut f),
            cnswap: num(&mut f),
            exit_signal: num(&mut f),
            processor: num(&mut f),
            rt_priority: num(&mut f),
            policy: num(&mut f),
            delayacct_blkio_ticks: num(&mut f),
            guest_time: num(&mut f),
            cguest_time: num(&mut f),
            start_data: num(&mut f),
            end_data: num(&mut f),
            start_brk: num(&mut f),
            arg_start: num(&mut f),
            arg_end: num(&mut f),
            env_start: num(&mut f),
            env_end: num(&mut f),
            exit_code: num(&mut f),
        }
    }
}

impl From<&str> for StatReading {
    fn from(s: &str) -> Self {
        StatReading::from(s.to_string())
    }
}