pub mod UnicodeCharGroup {

    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct UnicodeGroup {
        pub group: String,
        pub block: String
    }

    #[derive(Debug, Clone)]
    pub struct UnicodeStats {
        pub block: String,
        pub counter: u128,
        pub proportion: f32
    }

    #[derive(Debug)]
    struct GroupProfile {
        group: &'static str,
        full_name: &'static str,
        start: u32,
        end: u32
    }

    const GROUPS: [GroupProfile; 29] = [

        // Latin groups
        GroupProfile {
            group: "Latin",
            full_name: "Basic Latin",
            start: 0x0000,
            end: 0x007f
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin-1 Supplement",
            start: 0x0080,
            end: 0x00ff
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin Extended-A",
            start: 0x0100,
            end: 0x017f
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin Extended-B",
            start: 0x0180,
            end: 0x024f
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin Extended Additional",
            start: 0x1e00,
            end: 0x1eff
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin Extended-C",
            start: 0x2c60,
            end: 0x2c7f
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin Extended-D",
            start: 0xa720,
            end: 0xa7ff
        },
        GroupProfile {
            group: "Latin",
            full_name: "Latin Extended-E",
            start: 0xab30,
            end: 0xab6f
        },

        // Cyrillic groups
        GroupProfile {
            group: "Cyrillic",
            full_name: "Cyrillic",
            start: 0x0400,
            end: 0x04ff
        },
        GroupProfile {
            group: "Cyrillic",
            full_name: "Cyrillic Supplement",
            start: 0x0500,
            end: 0x052f
        },
        GroupProfile {
            group: "Cyrillic",
            full_name: "Cyrillic Extended-C",
            start: 0x1c80,
            end: 0x1c8f
        },
        GroupProfile {
            group: "Cyrillic",
            full_name: "Cyrillic Extended-A",
            start: 0x2de0,
            end: 0x2dff
        },
        GroupProfile {
            group: "Cyrillic",
            full_name: "Cyrillic Extended-B",
            start: 0xa640,
            end: 0xa69f
        },

        // CJK groups
        GroupProfile {
            group: "CJK",
            full_name: "CJK Radicals Supplement",
            start: 0x2e80,
            end: 0x2eff
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Symbols and Punctuation",
            start: 0x3000,
            end: 0x303f
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Strokes",
            start: 0x31c0,
            end: 0x31ef
        },
        GroupProfile {
            group: "CJK",
            full_name: "Enclosed CJK Letters and Months",
            start: 0x3200,
            end: 0x32ff
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Compatibility",
            start: 0x3300,
            end: 0x33ff
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension A",
            start: 0x3400,
            end: 0x4dbf
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs",
            start: 0x4e00,
            end: 0x9fff
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Compatibility Forms",
            start: 0xfe30,
            end: 0xfe4f
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension B",
            start: 0x20000,
            end: 0x2a6df
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension C",
            start: 0x2a700,
            end: 0x2b73f
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension D",
            start: 0x2b740,
            end: 0x2b81f
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension E",
            start: 0x2b820,
            end: 0x2ceaf
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension F",
            start: 0x2ceb0,
            end: 0x2ebef
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension F",
            start: 0x2ceb0,
            end: 0x2ebef
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Compatibility Ideographs Supplement",
            start: 0x2f800,
            end: 0x2fa1f
        },
        GroupProfile {
            group: "CJK",
            full_name: "CJK Unified Ideographs Extension G",
            start: 0x30000,
            end: 0x3134f
        }
    ];


    pub fn detect_char(ch: char) -> Option<UnicodeGroup> {
        let code = ch as u32;
        for potential_group in GROUPS {
            let (start, end) = (potential_group.start, potential_group.end);
            if end >= code && code >= start {
                return Some(UnicodeGroup {
                    group: String::from(potential_group.group),
                    block: String::from(potential_group.full_name.clone())
                })
            }
        }
        None
    }

    pub fn stats(text: &str) -> HashMap<String, Vec<UnicodeStats>> {
        let mut res: HashMap<String, Vec<UnicodeStats>> = HashMap::new();
        let total = text.chars().count();
        for ch in text.chars() {
            if let Some(tmp) = detect_char(ch) {
                let (group, block) = (tmp.group, tmp.block);
                let mut v = res
                    .entry(group)
                    .or_insert_with(|| vec![
                        UnicodeStats {
                            block: block.clone(),
                            counter: 1,
                            proportion: (1. / total as f32) * 100.
                        }
                    ]
                );

                if let Some(el) = v.into_iter().find(|o| o.block == block) {
                    el.counter += 1;
                    el.proportion = (el.counter as f32 / total as f32) * 100.
                } else {
                    v.push(UnicodeStats {
                        block: block.clone(),
                        counter: 1,
                        proportion: (1. / total as f32) * 100.
                    });
                }
            }
        }
        res
    }

}