use std::{fmt, fmt::Write, io};

use ntex_bytes::ByteString;

#[allow(clippy::match_same_arms)]
pub(crate) fn is_valid(topic: &str) -> bool { panic!("STUB: not implemented") }

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TopicFilterError {
    InvalidTopic,
    InvalidLevel,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TopicFilterLevel {
    Normal(ByteString),
    System(ByteString),
    Blank,
    SingleWildcard, 
    MultiWildcard,  
}

impl TopicFilterLevel {
    fn is_valid(&self) -> bool { panic!("STUB: not implemented") }
}

fn match_topic<T: MatchLevel, L: Iterator<Item = T>>(
    superset: &TopicFilter,
    subset: L,
) -> bool { panic!("STUB: not implemented") }

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicFilter(Vec<TopicFilterLevel>);

impl TopicFilter {
    pub fn levels(&self) -> &[TopicFilterLevel] { panic!("STUB: not implemented") }

    fn is_valid(&self) -> bool { panic!("STUB: not implemented") }

    pub fn matches_filter(&self, topic: &TopicFilter) -> bool { panic!("STUB: not implemented") }

    pub fn matches_topic<S: AsRef<str> + ?Sized>(&self, topic: &S) -> bool { panic!("STUB: not implemented") }
}

impl TryFrom<&[TopicFilterLevel]> for TopicFilter {
    type Error = TopicFilterError;

    fn try_from(s: &[TopicFilterLevel]) -> Result<Self, Self::Error> { panic!("STUB: not implemented") }
}

impl TryFrom<Vec<TopicFilterLevel>> for TopicFilter {
    type Error = TopicFilterError;

    fn try_from(v: Vec<TopicFilterLevel>) -> Result<Self, Self::Error> { panic!("STUB: not implemented") }
}

impl From<TopicFilter> for Vec<TopicFilterLevel> {
    fn from(t: TopicFilter) -> Self { panic!("STUB: not implemented") }
}

trait MatchLevel {
    fn match_level(&self, level: &TopicFilterLevel, index: usize) -> bool;
}

impl MatchLevel for TopicFilterLevel {
    fn match_level(&self, level: &TopicFilterLevel, index: usize) -> bool { panic!("STUB: not implemented") }
}

impl MatchLevel for &TopicFilterLevel {
    fn match_level(&self, level: &TopicFilterLevel, index: usize) -> bool { panic!("STUB: not implemented") }
}

fn match_level_impl(
    subset_level: &TopicFilterLevel,
    superset_level: &TopicFilterLevel,
    _index: usize,
) -> bool { panic!("STUB: not implemented") }

impl<T: AsRef<str>> MatchLevel for T {
    fn match_level(&self, level: &TopicFilterLevel, index: usize) -> bool { panic!("STUB: not implemented") }
}

impl TryFrom<ByteString> for TopicFilter {
    type Error = TopicFilterError;

    fn try_from(value: ByteString) -> Result<Self, Self::Error> { panic!("STUB: not implemented") }
}

impl std::str::FromStr for TopicFilter {
    type Err = TopicFilterError;

    fn from_str(value: &str) -> Result<Self, Self::Err> { panic!("STUB: not implemented") }
}

impl fmt::Display for TopicFilterLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

impl fmt::Display for TopicFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { panic!("STUB: not implemented") }
}

#[allow(dead_code)]
pub(crate) trait WriteTopicExt: io::Write {
    fn write_level(&mut self, level: &TopicFilterLevel) -> io::Result<usize> { panic!("STUB: not implemented") }

    fn write_topic(&mut self, topic: &TopicFilter) -> io::Result<usize> { panic!("STUB: not implemented") }
}

impl<W: io::Write + ?Sized> WriteTopicExt for W {}

fn is_system<T: AsRef<str>>(s: T) -> bool { panic!("STUB: not implemented") }

fn recover_bstr(superset: &ByteString, subset: &str) -> ByteString { panic!("STUB: not implemented") }

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abc" => true; "pass_norm1")]
    #[test_case("a/b" => true; "pass_norm2")]
    #[test_case("/" => true; "pass_norm3")]
    #[test_case("//" => true; "pass_norm4")]
    #[test_case("a/b/+" => true; "pass_plus1")]
    #[test_case("+/a" => true; "pass_plus2")]
    #[test_case("+" => true; "pass_plus3")]
    #[test_case("+//+" => true; "pass_plus4")]
    #[test_case("a/b/#" => true; "pass_hash1")]
    #[test_case("#" => true; "pass_hash2")]
    #[test_case("/#" => true; "pass_hash3")]
    #[test_case("++" => false; "fail_plus1")]
    #[test_case("b+/" => false; "fail_plus2")]
    #[test_case("a/+b" => false; "fail_plus3")]
    #[test_case("+#" => false; "fail_hash1")]
    #[test_case("a#" => false; "fail_hash2")]
    #[test_case("a/#/" => false; "fail_hash3")]
    #[test_case("a/#b" => false; "fail_hash4")]
    #[test_case("a/##" => false; "fail_hash5")]
    #[test_case("a/#+" => false; "fail_hash6")]
    fn check_is_valid(topic_filter: &'static str) -> bool {
        is_valid(topic_filter)
    }

    fn lvl_normal<T: AsRef<str>>(s: T) -> TopicFilterLevel {
        assert!(
            !s.as_ref().contains(['+', '#']),
            "invalid normal level `{}` contains +|#",
            s.as_ref()
        );
        TopicFilterLevel::Normal(s.as_ref().into())
    }

    fn lvl_sys<T: AsRef<str>>(s: T) -> TopicFilterLevel {
        assert!(
            !s.as_ref().contains(['+', '#']),
            "invalid normal level `{}` contains +|#",
            s.as_ref()
        );
        assert!(
            s.as_ref().starts_with('$'),
            "invalid metadata level `{}` not starts with $",
            s.as_ref()
        );
        TopicFilterLevel::System(s.as_ref().into())
    }

    fn topic(topic: &'static str) -> TopicFilter {
        TopicFilter::try_from(ByteString::from_static(topic)).unwrap()
    }

    #[test_case("level" => Ok(vec![lvl_normal("level")]) ; "1")]
    #[test_case("level/+" => Ok(vec![lvl_normal("level"), TopicFilterLevel::SingleWildcard]) ; "2")]
    #[test_case("a//#" => Ok(vec![lvl_normal("a"), TopicFilterLevel::Blank, TopicFilterLevel::MultiWildcard]) ; "3")]
    #[test_case("$a///#" => Ok(vec![lvl_sys("$a"), TopicFilterLevel::Blank, TopicFilterLevel::Blank, TopicFilterLevel::MultiWildcard]) ; "4")]
    #[test_case("$a/#/" => Err(TopicFilterError::InvalidTopic) ; "5")]
    #[test_case("a+b" => Err(TopicFilterError::InvalidLevel) ; "6")]
    #[test_case("a/+b" => Err(TopicFilterError::InvalidLevel) ; "7")]
    #[test_case("$a/$b/" => Ok(vec![lvl_sys("$a"), lvl_normal("$b"), TopicFilterLevel::Blank]) ; "8")]
    #[test_case("#/a" => Err(TopicFilterError::InvalidTopic) ; "10")]
    #[test_case("" => Err(TopicFilterError::InvalidTopic) ; "11")]
    #[test_case("/finance" => Ok(vec![TopicFilterLevel::Blank, lvl_normal("finance")]) ; "12")]
    #[test_case("finance/" => Ok(vec![lvl_normal("finance"), TopicFilterLevel::Blank]) ; "13")]
    fn parsing(input: &str) -> Result<Vec<TopicFilterLevel>, TopicFilterError> {
        TopicFilter::try_from(ByteString::from(input)).map(|t| t.levels().to_vec())
    }

    #[test_case(vec![lvl_normal("sport"), lvl_normal("tennis"), lvl_normal("player1")] => true; "1")]
    #[test_case(vec![lvl_normal("sport"), lvl_normal("tennis"), TopicFilterLevel::MultiWildcard] => true; "2")]
    #[test_case(vec![lvl_sys("$SYS"), lvl_normal("tennis"), lvl_normal("player1")] => true; "3")]
    #[test_case(vec![lvl_normal("sport"), TopicFilterLevel::SingleWildcard, lvl_normal("player1")] => true; "4")]
    #[test_case(vec![lvl_normal("sport"), TopicFilterLevel::MultiWildcard, lvl_normal("player1")] => false; "5")]
    #[test_case(vec![lvl_normal("sport"), lvl_sys("$SYS"), lvl_normal("player1")] => false; "6")]
    fn topic_is_valid(levels: Vec<TopicFilterLevel>) -> bool {
        TopicFilter::try_from(levels).is_ok()
    }

    #[test]
    fn test_multi_wildcard_topic() {
        assert!(topic("sport/tennis/#").matches_filter(&TopicFilter(vec![
            lvl_normal("sport"),
            lvl_normal("tennis"),
            TopicFilterLevel::MultiWildcard
        ])));

        assert!(topic("sport/tennis/#").matches_topic("sport/tennis"));

        assert!(topic("#").matches_filter(&TopicFilter(vec![TopicFilterLevel::MultiWildcard])));
    }

    #[test]
    fn test_single_wildcard_topic() {
        assert!(topic("+").matches_filter(
            &TopicFilter::try_from(vec![TopicFilterLevel::SingleWildcard]).unwrap()
        ));

        assert!(topic("+/tennis/#").matches_filter(&TopicFilter(vec![
            TopicFilterLevel::SingleWildcard,
            lvl_normal("tennis"),
            TopicFilterLevel::MultiWildcard
        ])));

        assert!(topic("sport/+/player1").matches_filter(&TopicFilter(vec![
            lvl_normal("sport"),
            TopicFilterLevel::SingleWildcard,
            lvl_normal("player1")
        ])));
    }

    #[test]
    fn test_write_topic() {
        let mut v = vec![];
        let t = TopicFilter(vec![
            TopicFilterLevel::SingleWildcard,
            lvl_normal("tennis"),
            TopicFilterLevel::MultiWildcard,
        ]);

        assert_eq!(v.write_topic(&t).unwrap(), 10);
        assert_eq!(v, b"+/tennis/#");

        assert_eq!(format!("{t}"), "+/tennis/#");
        assert_eq!(t.to_string(), "+/tennis/#");
    }

    #[test_case("test", "test" => true)]
    #[test_case("$SYS", "$SYS" => true)]
    #[test_case("sport/tennis/player1/#", "sport/tennis/player1" => true)]
    #[test_case("sport/tennis/player1/#", "sport/tennis/player1/score" => true)]
    #[test_case("sport/tennis/player1/#", "sport/tennis/player1/score/wimbledon" => true)]
    #[test_case("sport/#", "sport" => true)]
    #[test_case("sport/tennis/+", "sport/tennis/player1" => true)]
    #[test_case("sport/tennis/+", "sport/tennis/player2" => true)]
    #[test_case("sport/tennis/+", "sport/tennis/player1/ranking" => false)]
    #[test_case("sport/+", "sport" => false; "single1")]
    #[test_case("sport/+", "sport/" => true; "single2")]
    #[test_case("+/+", "/finance" => true; "single3")]
    #[test_case("/+", "/finance" => true; "single4")]
    #[test_case("+", "/finance" => false; "single5")]
    #[test_case("#", "$SYS" => false; "sys1")]
    #[test_case("+/monitor/Clients", "$SYS/monitor/Clients" => false; "sys2")]
    #[test_case("$SYS/#", "$SYS/" => true; "sys3")]
    #[test_case("$SYS/monitor/+", "$SYS/monitor/Clients" => true; "sys4")]
    #[test_case("#", "/$SYS/monitor/Clients" => true; "sys5")]
    #[test_case("+", "$SYS" => false; "sys6")]
    #[test_case("+/#", "$SYS" => false; "sys7")]
    fn matches_topic(filter: &'static str, topic_str: &'static str) -> bool {
        topic(filter).matches_topic(topic_str)
    }

    #[test_case("a/b", "a/b" => true; "1")]
    #[test_case("a/b", "a/+" => false; "2")]
    #[test_case("a/b", "a/#" => false; "3")]
    #[test_case("a/+", "a/#" => false; "4")]
    #[test_case("a/+", "a/b" => true; "5")]
    #[test_case("+/+", "/" => true; "6")]
    #[test_case("+/+", "#" => false; "7")]
    #[test_case("+", "#" => false; "8")]
    #[test_case("#", "+" => true; "9")]
    #[test_case("#", "#" => true; "10")]
    #[test_case("a/#", "a/+/+" => true; "11")]
    #[test_case("a/+/normal/+", "a/$not_sys/normal/+" => true; "12")]
    #[test_case("a/+/#", "a/b" => true; "13")]
    fn matches_filter(superset_filter: &'static str, subset_filter: &'static str) -> bool {
        topic(superset_filter).matches_filter(&topic(subset_filter))
    }
}
