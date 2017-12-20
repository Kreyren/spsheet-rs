//! Excel Base Format Style
use std::borrow::Cow;
use super::nom::{IResult};

#[derive(Debug, Clone, PartialEq)]
pub struct Style {
    format: String,
}

impl Style {
    pub fn new<'a, S>(format: S) -> Style 
         where S: Into<Cow<'a, str>>
    {
        Style {
            format: format.into().into_owned(),
        }
    }

    pub fn get_format(&self) -> &String {
        &self.format
    }

    pub fn get_date_formats(&self) -> Option<Vec<&str>> {
        match ymdhms(self.format.as_str()) {
            IResult::Done(_, output) => {
                let mut result = vec![];
                for i in output {
                    for j in i {
                        result.push(j);
                    }
                }
                Some(result)
            },
            _ => None
        }
    }
}

named!(year4<&str, &str>, 
    map!(alt!(tag!("yyyy") | tag!("YYYY")), |_| "%Y"));

named!(year2<&str, &str>, 
    map!(alt!(tag!("yy") | tag!("YY")), |_| "%y"));

named!(era1<&str, &str>, 
    map!(alt!(tag!("e") | tag!("E")), |_| "{{era1}}"));

named!(era2<&str, &str>, 
    map!(alt!(tag!("ee") | tag!("EE")), |_| "{{era2}}"));

named!(gengou1<&str, &str>, 
    map!(alt!(tag!("g") | tag!("G")), |_| "{{gengou1}}"));

named!(gengou2<&str, &str>, 
    map!(alt!(tag!("gg") | tag!("GG")), |_| "{{gengou2}}"));

named!(gengou3<&str, &str>, 
    map!(alt!(tag!("ggg") | tag!("GGG")), |_| "{{gengou3}}"));

named!(year<&str, &str>,
    alt!(
        complete!(year4) | 
        complete!(year2) | 
        complete!(era2) | 
        complete!(era1) | 
        complete!(gengou3) | 
        complete!(gengou2) | 
        complete!(gengou1)
    )
);

named!(month1<&str, &str>, 
    map!(alt!(tag!("m") | tag!("M")), |_| "%-m"));

named!(month2<&str, &str>, 
    map!(alt!(tag!("mm") | tag!("MM")), |_| "%m"));

named!(month3<&str, &str>, 
    map!(alt!(tag!("mmm") | tag!("MMM")), |_| "%b"));

named!(month4<&str, &str>, 
    map!(alt!(tag!("mmmm") | tag!("MMMM")), |_| "%B"));

named!(month5<&str, &str>, 
    map!(alt!(tag!("mmmmm") | tag!("MMMMM")), |_| "{{month5}}"));

named!(month<&str, &str>,
    alt!(
        complete!(month5) | 
        complete!(month4) | 
        complete!(month3) | 
        complete!(month2) | 
        complete!(month1)
    )
);

named!(day1<&str, &str>, 
    map!(alt!(tag!("d") | tag!("D")), |_| "%-d"));

named!(day2<&str, &str>, 
    map!(alt!(tag!("dd") | tag!("DD")), |_| "%d"));

named!(dow3<&str, &str>, 
    map!(alt!(tag!("ddd") | tag!("DDD")), |_| "%a"));

named!(dow4<&str, &str>, 
    map!(alt!(tag!("dddd") | tag!("DDDD")), |_| "%A"));

named!(youbi3<&str, &str>, 
    map!(alt!(tag!("aaa") | tag!("AAA")), |_| "{{youbi3}}"));

named!(youbi4<&str, &str>, 
    map!(alt!(tag!("aaaa") | tag!("AAAA")), |_| "{{youbi4}}"));

named!(day<&str, &str>,
    alt!(
        complete!(youbi4) | 
        complete!(youbi3) | 
        complete!(dow4) | 
        complete!(dow3) | 
        complete!(day2) | 
        complete!(day1)
    )
);

named!(hour1<&str, &str>, 
    map!(alt!(tag!("h") | tag!("H")), |_| "%-H"));

named!(hour2<&str, &str>, 
    map!(alt!(tag!("hh") | tag!("HH")), |_| "%H"));

named!(hour<&str, &str>, 
    alt!(
        complete!(hour2) | 
        complete!(hour1)
    )
);

named!(minute1<&str, &str>, 
    map!(alt!(tag!("m") | tag!("M")), |_| "%-M"));

named!(minute2<&str, &str>, 
    map!(alt!(tag!("mm") | tag!("MM")), |_| "%M"));

named!(minute<&str, &str>, 
    alt!(
        complete!(minute2) | 
        complete!(minute1)
    )
);

named!(second1<&str, &str>, 
    map!(alt!(tag!("s") | tag!("S")), |_| "%-S"));

named!(second2<&str, &str>, 
    map!(alt!(tag!("ss") | tag!("SS")), |_| "%S"));

named!(second<&str, &str>, 
    alt!(
        complete!(second2) | 
        complete!(second1)
    )
);

named!(special_word<&str, &str>, 
    map!(alt!(tag!("/") | tag!(":")), |x| x));

named!(escaped_word<&str, &str>, 
    do_parse!(
        tag!("\\") >>
        res: take_s!(1) >>
        (res)
    ));

named!(quoted_word<&str, &str>,
    delimited!(tag!("\""), take_until_s!("\""), tag!("\""))
);

named!(word<&str, &str>, 
    alt!(
        complete!(quoted_word) | 
        complete!(escaped_word) | 
        complete!(special_word)
    )
);

named!(hm<&str, Vec<&str> >,
    do_parse!(
        h: hour >>
        w: opt!(word) >>
        m: minute >>
        (
            if let Some(w) = w {
                vec![h, w ,m] 
            } else {
                 vec![h, m] 
            }
        )
    )
);

named!(ms<&str, Vec<&str> >,
    do_parse!(
        m: minute >>
        w: opt!(word) >>
        s: second >>
        (
            if let Some(w) = w {
                vec![m, w ,s] 
            } else {
                 vec![m, s] 
            }
        )
    )
);

named!(ymdhms<&str, Vec<Vec<&str>> >,
    many0!(alt!(hm | ms | 
        map!(second, |x| vec![x]) |
        map!(hour, |x| vec![x]) |
        map!(year, |x| vec![x]) |
        map!(month, |x| vec![x]) |
        map!(day, |x| vec![x]) |
        map!(word, |x| vec![x])
    ))
);