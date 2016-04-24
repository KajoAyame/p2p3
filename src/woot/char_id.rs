#![allow(dead_code,unused_variables,unused_imports)]

use std::cmp::Ordering;

#[derive(Clone,PartialEq,Debug,RustcDecodable, RustcEncodable)]
pub enum CharId {
    Beginning,
    Ending,
    Regular {site_id: u32, unique_id: u32},// UniqueId is the logicalClock value at the time of creation
}

impl PartialOrd<CharId> for CharId {
    fn lt(&self, other: &CharId) -> bool {
        let cloned_self = self.clone();
        let cloned_other = other.clone();
        return match cloned_other {
            CharId::Beginning => false,
            CharId::Ending => true,
            CharId::Regular {site_id, unique_id} => {
                let other_site_id = site_id;
                let other_unique_id = unique_id;
                match cloned_self {
                    CharId::Beginning => true,
                    CharId::Ending => false,
                    CharId::Regular {site_id, unique_id} => {
                        if (site_id < other_site_id) || (site_id == other_site_id && unique_id < other_unique_id) {
                            true
                        } else {
                            false
                        }
                    }
                }
            }
        }
    }

    fn partial_cmp(&self, other: &CharId) -> Option<Ordering> {
        let cloned_self = self.clone();
        let cloned_other = other.clone();
        return match cloned_other {
            CharId::Beginning => Some(Ordering:: Greater),
            CharId::Ending => Some(Ordering:: Less),
            CharId::Regular {site_id, unique_id} => {
                let other_site_id = site_id;
                let other_unique_id = unique_id;
                match cloned_self {
                    CharId::Beginning => Some(Ordering:: Less),
                    CharId::Ending => Some(Ordering:: Greater),
                    CharId::Regular {site_id, unique_id} => {
                        if (site_id < other_site_id) || (site_id == other_site_id && unique_id < other_unique_id) {
                            Some(Ordering:: Less)
                        } else {
                            Some(Ordering:: Greater)
                        }
                    }
                }
            }
        };
    }
}

pub fn create_char_id(site_id: u32, unique_id: u32) -> CharId {
    CharId::Regular {site_id: site_id, unique_id: unique_id}
}

#[test]
fn test_id_comparison() {
    let char_id_beg = CharId::Beginning;
    let char_id_end = CharId::Ending;
    let char_id_1 = create_char_id(0, 0);
    let char_id_2 = create_char_id(1, 0);
    let char_id_3 = create_char_id(0, 1);
    let char_id_4 = create_char_id(1, 1);
    let char_id_5 = create_char_id(0, 5);
    assert_eq!(char_id_beg < char_id_end, true);
    assert_eq!(char_id_end < char_id_beg, false);
    assert_eq!(char_id_1 < char_id_2, true);
    assert_eq!(char_id_2 < char_id_3, false);
    assert_eq!(char_id_5 < char_id_end, true);
    assert_eq!(char_id_beg < char_id_5, true);
    assert_eq!(char_id_2 < char_id_4, true);
    assert_eq!(char_id_5 < char_id_2, true);

}
