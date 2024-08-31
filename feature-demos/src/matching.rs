struct Inner {
    i_label: String,
    i_num: i32,
    i_opt: Option<String>,
}

struct Middle {
    m_label: String,
    m_num: i32,
    m_inner: Inner,
}

struct Outer {
    o_label: String,
    o_num: i32,
    o_middle: Middle,
}

fn get_labels(o: &Outer) -> (&String, &String, &String) {
    let Outer {
        o_label: outer_label,
        o_middle:
            Middle {
                m_label: middle_label,
                m_inner:
                    Inner {
                        i_label: inner_label,
                        ..
                    },
                ..
            },
        ..
    } = o;
    (outer_label, middle_label, inner_label)
}

fn get_inner_opt(o: &Outer) -> &str {
    match o.o_middle.m_inner.i_opt.as_ref() {
        Some(v) => v.as_str(),
        None => "",
    }
}

pub fn run() {
    let mut o = Outer {
        o_label: "The Outer Struct".to_string(),
        o_num: 42,
        o_middle: Middle {
            m_label: "The Middle Struct".to_string(),
            m_num: 1728,
            m_inner: Inner {
                i_label: "The Inner Struct".to_string(),
                i_num: 720,
                i_opt: None,
            },
        },
    };

    let labels = get_labels(&o);
    println!("LABELS: {:?}", labels);

    let i_opt = get_inner_opt(&o);
    println!("I OPT UNSET: {}", i_opt);

    o.o_middle.m_inner.i_opt = Some(String::from("foo"));
    let i_opt = get_inner_opt(&o);
    println!("I OPT IS SET: {}", i_opt);
}
