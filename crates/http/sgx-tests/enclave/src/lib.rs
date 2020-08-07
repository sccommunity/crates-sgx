#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
use std::prelude::v1::*;
use crates_unittest::{ run_inventory_tests, test_case };
use http::header::*;
use http::*;
use std::string::ToString;

#[test_case]
fn from_bytes() {
    for ok in &[
        "100", "101", "199", "200", "250", "299", "321", "399", "499", "599",
    ] {
        assert!(StatusCode::from_bytes(ok.as_bytes()).is_ok());
    }

    for not_ok in &[
        "0", "00", "10", "40", "99", "000", "010", "099", "600", "610", "999",
    ] {
        assert!(StatusCode::from_bytes(not_ok.as_bytes()).is_err());
    }
}

#[test_case]
fn equates_with_u16() {
    let status = StatusCode::from_u16(200u16).unwrap();
    assert_eq!(200u16, status);
    assert_eq!(status, 200u16);
}

macro_rules! test_round_trip {
    ($($num:expr,)+) => {
        #[test]
        fn roundtrip() {
            $(
                let status = StatusCode::from_bytes(stringify!($num).as_bytes()).unwrap();
                let expect = $num;

                assert_eq!(u16::from(status), expect);
            )+
        }
    }
}

test_round_trip!(
    100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
    120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
    140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159,
    160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179,
    180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199,

    200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219,
    220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239,
    240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, 257, 258, 259,
    260, 261, 262, 263, 264, 265, 266, 267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279,
    280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299,

    300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319,
    320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339,
    340, 341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359,
    360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379,
    380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, 392, 393, 394, 395, 396, 397, 398, 399,

    400, 401, 402, 403, 404, 405, 406, 407, 408, 409, 410, 411, 412, 413, 414, 415, 416, 417, 418, 419,
    420, 421, 422, 423, 424, 425, 426, 427, 428, 429, 430, 431, 432, 433, 434, 435, 436, 437, 438, 439,
    440, 441, 442, 443, 444, 445, 446, 447, 448, 449, 450, 451, 452, 453, 454, 455, 456, 457, 458, 459,
    460, 461, 462, 463, 464, 465, 466, 467, 468, 469, 470, 471, 472, 473, 474, 475, 476, 477, 478, 479,
    480, 481, 482, 483, 484, 485, 486, 487, 488, 489, 490, 491, 492, 493, 494, 495, 496, 497, 498, 499,

    500, 501, 502, 503, 504, 505, 506, 507, 508, 509, 510, 511, 512, 513, 514, 515, 516, 517, 518, 519,
    520, 521, 522, 523, 524, 525, 526, 527, 528, 529, 530, 531, 532, 533, 534, 535, 536, 537, 538, 539,
    540, 541, 542, 543, 544, 545, 546, 547, 548, 549, 550, 551, 552, 553, 554, 555, 556, 557, 558, 559,
    560, 561, 562, 563, 564, 565, 566, 567, 568, 569, 570, 571, 572, 573, 574, 575, 576, 577, 578, 579,
    580, 581, 582, 583, 584, 585, 586, 587, 588, 589, 590, 591, 592, 593, 594, 595, 596, 597, 598, 599,
    );


#[test_case]
fn smoke() {
    let mut headers = HeaderMap::new();

    assert!(headers.get("hello").is_none());

    let name: HeaderName = "hello".parse().unwrap();

    match headers.entry(&name) {
        Entry::Vacant(e) => {
            e.insert("world".parse().unwrap());
        }
        _ => panic!(),
    }

    assert!(headers.get("hello").is_some());

    match headers.entry(&name) {
        Entry::Occupied(mut e) => {
            assert_eq!(e.get(), &"world");

            // Push another value
            e.append("zomg".parse().unwrap());

            let mut i = e.iter();

            assert_eq!(*i.next().unwrap(), "world");
            assert_eq!(*i.next().unwrap(), "zomg");
            assert!(i.next().is_none());
        }
        _ => panic!(),
    }
}

//#[test_case]
//#[should_panic]
fn reserve_over_capacity() {
    // See https://github.com/hyperium/http/issues/352
    let mut headers = HeaderMap::<u32>::with_capacity(32);
    headers.reserve(50_000); // over MAX_SIZE
}

#[test_case]
fn with_capacity_max() {
    // The largest capacity such that (cap + cap / 3) < MAX_SIZE.
    HeaderMap::<u32>::with_capacity(24_576);
}

//#[test_case]
//#[should_panic]
fn with_capacity_overflow() {
    HeaderMap::<u32>::with_capacity(24_577);
}

//#[test_case]
//#[should_panic]
fn reserve_overflow() {
    // See https://github.com/hyperium/http/issues/352
    let mut headers = HeaderMap::<u32>::with_capacity(0);
    headers.reserve(std::usize::MAX); // next_power_of_two overflows
}

#[test_case]
fn drain() {
    let mut headers = HeaderMap::new();

    // Insert a single value
    let name: HeaderName = "hello".parse().unwrap();
    headers.insert(name, "world".parse().unwrap());

    {
        let mut iter = headers.drain();
        let (name, value) = iter.next().unwrap();
        assert_eq!(name.unwrap().as_str(), "hello");

        assert_eq!(value, "world");

        assert!(iter.next().is_none());
    }

    assert!(headers.is_empty());

    // Insert two sequential values
    headers.insert(
        "hello".parse::<HeaderName>().unwrap(),
        "world".parse().unwrap(),
    );
    headers.insert(
        "zomg".parse::<HeaderName>().unwrap(),
        "bar".parse().unwrap(),
    );
    headers.append(
        "hello".parse::<HeaderName>().unwrap(),
        "world2".parse().unwrap(),
    );

    // Drain...
    {
        let mut iter = headers.drain();

        let (name, value) = iter.next().unwrap();
        assert_eq!(name.unwrap().as_str(), "hello");
        assert_eq!(value, "world");

        let (name, value) = iter.next().unwrap();
        assert_eq!(name, None);
        assert_eq!(value, "world2");

        let (name, value) = iter.next().unwrap();
        assert_eq!(name.unwrap().as_str(), "zomg");
        assert_eq!(value, "bar");

        assert!(iter.next().is_none());
    }
}

#[test_case]
fn drain_drop_immediately() {
    // test mem::forgetting does not double-free

    let mut headers = HeaderMap::new();
    headers.insert("hello", "world".parse().unwrap());
    headers.insert("zomg", "bar".parse().unwrap());
    headers.append("hello", "world2".parse().unwrap());

    let iter = headers.drain();
    assert_eq!(iter.size_hint(), (2, Some(3)));
    // not consuming `iter`
}

#[test_case]
fn drain_forget() {
    // test mem::forgetting does not double-free

    let mut headers = HeaderMap::<HeaderValue>::new();
    headers.insert("hello", "world".parse().unwrap());
    headers.insert("zomg", "bar".parse().unwrap());

    assert_eq!(headers.len(), 2);

    {
        let mut iter = headers.drain();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        let _ = iter.next().unwrap();
        std::mem::forget(iter);
    }

    assert_eq!(headers.len(), 0);
}

#[test_case]
fn drain_entry() {
    let mut headers = HeaderMap::new();

    headers.insert(
        "hello".parse::<HeaderName>().unwrap(),
        "world".parse().unwrap(),
    );
    headers.insert(
        "zomg".parse::<HeaderName>().unwrap(),
        "foo".parse().unwrap(),
    );
    headers.append(
        "hello".parse::<HeaderName>().unwrap(),
        "world2".parse().unwrap(),
    );
    headers.insert(
        "more".parse::<HeaderName>().unwrap(),
        "words".parse().unwrap(),
    );
    headers.append(
        "more".parse::<HeaderName>().unwrap(),
        "insertions".parse().unwrap(),
    );

    // Using insert
    {
        let mut e = match headers.entry("hello") {
            Entry::Occupied(e) => e,
            _ => panic!(),
        };

        let vals: Vec<_> = e.insert_mult("wat".parse().unwrap()).collect();
        assert_eq!(2, vals.len());
        assert_eq!(vals[0], "world");
        assert_eq!(vals[1], "world2");
    }
}

#[test_case]
fn eq() {
    let mut a = HeaderMap::new();
    let mut b = HeaderMap::new();

    assert_eq!(a, b);

    a.insert(
        "hello".parse::<HeaderName>().unwrap(),
        "world".parse().unwrap(),
    );
    assert_ne!(a, b);

    b.insert(
        "hello".parse::<HeaderName>().unwrap(),
        "world".parse().unwrap(),
    );
    assert_eq!(a, b);

    a.insert("foo".parse::<HeaderName>().unwrap(), "bar".parse().unwrap());
    a.append("foo".parse::<HeaderName>().unwrap(), "baz".parse().unwrap());
    assert_ne!(a, b);

    b.insert("foo".parse::<HeaderName>().unwrap(), "bar".parse().unwrap());
    assert_ne!(a, b);

    b.append("foo".parse::<HeaderName>().unwrap(), "baz".parse().unwrap());
    assert_eq!(a, b);

    a.append("a".parse::<HeaderName>().unwrap(), "a".parse().unwrap());
    a.append("a".parse::<HeaderName>().unwrap(), "b".parse().unwrap());
    b.append("a".parse::<HeaderName>().unwrap(), "b".parse().unwrap());
    b.append("a".parse::<HeaderName>().unwrap(), "a".parse().unwrap());

    assert_ne!(a, b);
}

#[test_case]
fn into_header_name() {
    let mut m = HeaderMap::new();
    m.insert(HOST, "localhost".parse().unwrap());
    m.insert(&ACCEPT, "*/*".parse().unwrap());
    m.insert("connection", "keep-alive".parse().unwrap());

    m.append(LOCATION, "/".parse().unwrap());
    m.append(&VIA, "bob".parse().unwrap());
    m.append("transfer-encoding", "chunked".parse().unwrap());

    assert_eq!(m.len(), 6);
}

#[test_case]
fn as_header_name() {
    let mut m = HeaderMap::new();
    let v: HeaderValue = "localhost".parse().unwrap();
    m.insert(HOST, v.clone());

    let expected = Some(&v);

    assert_eq!(m.get("host"), expected);
    assert_eq!(m.get(&HOST), expected);

    let s = String::from("host");
    assert_eq!(m.get(&s), expected);
    assert_eq!(m.get(s.as_str()), expected);
}

#[test_case]
fn insert_all_std_headers() {
    let mut m = HeaderMap::new();

    for (i, hdr) in STD.iter().enumerate() {
        m.insert(hdr.clone(), hdr.as_str().parse().unwrap());

        for j in 0..(i + 1) {
            assert_eq!(m[&STD[j]], STD[j].as_str());
        }

        if i != 0 {
            for j in (i + 1)..STD.len() {
                assert!(
                    m.get(&STD[j]).is_none(),
                    "contained {}; j={}",
                    STD[j].as_str(),
                    j
                );
            }
        }
    }
}

#[test_case]
fn insert_79_custom_std_headers() {
    let mut h = HeaderMap::new();
    let hdrs = custom_std(79);

    for (i, hdr) in hdrs.iter().enumerate() {
        h.insert(hdr.clone(), hdr.as_str().parse().unwrap());

        for j in 0..(i + 1) {
            assert_eq!(h[&hdrs[j]], hdrs[j].as_str());
        }

        for j in (i + 1)..hdrs.len() {
            assert!(h.get(&hdrs[j]).is_none());
        }
    }
}

#[test_case]
fn append_multiple_values() {
    let mut map = HeaderMap::new();

    map.append(header::CONTENT_TYPE, "json".parse().unwrap());
    map.append(header::CONTENT_TYPE, "html".parse().unwrap());
    map.append(header::CONTENT_TYPE, "xml".parse().unwrap());

    let vals = map
        .get_all(&header::CONTENT_TYPE)
        .iter()
        .collect::<Vec<_>>();

    assert_eq!(&vals, &[&"json", &"html", &"xml"]);
}

fn custom_std(n: usize) -> Vec<HeaderName> {
    (0..n)
        .map(|i| {
            let s = format!("{}-{}", STD[i % STD.len()].as_str(), i);
            s.parse().unwrap()
        })
        .collect()
}

const STD: &'static [HeaderName] = &[
    ACCEPT,
    ACCEPT_CHARSET,
    ACCEPT_ENCODING,
    ACCEPT_LANGUAGE,
    ACCEPT_RANGES,
    ACCESS_CONTROL_ALLOW_CREDENTIALS,
    ACCESS_CONTROL_ALLOW_HEADERS,
    ACCESS_CONTROL_ALLOW_METHODS,
    ACCESS_CONTROL_ALLOW_ORIGIN,
    ACCESS_CONTROL_EXPOSE_HEADERS,
    ACCESS_CONTROL_MAX_AGE,
    ACCESS_CONTROL_REQUEST_HEADERS,
    ACCESS_CONTROL_REQUEST_METHOD,
    AGE,
    ALLOW,
    ALT_SVC,
    AUTHORIZATION,
    CACHE_CONTROL,
    CONNECTION,
    CONTENT_DISPOSITION,
    CONTENT_ENCODING,
    CONTENT_LANGUAGE,
    CONTENT_LENGTH,
    CONTENT_LOCATION,
    CONTENT_RANGE,
    CONTENT_SECURITY_POLICY,
    CONTENT_SECURITY_POLICY_REPORT_ONLY,
    CONTENT_TYPE,
    COOKIE,
    DNT,
    DATE,
    ETAG,
    EXPECT,
    EXPIRES,
    FORWARDED,
    FROM,
    HOST,
    IF_MATCH,
    IF_MODIFIED_SINCE,
    IF_NONE_MATCH,
    IF_RANGE,
    IF_UNMODIFIED_SINCE,
    LAST_MODIFIED,
    LINK,
    LOCATION,
    MAX_FORWARDS,
    ORIGIN,
    PRAGMA,
    PROXY_AUTHENTICATE,
    PROXY_AUTHORIZATION,
    PUBLIC_KEY_PINS,
    PUBLIC_KEY_PINS_REPORT_ONLY,
    RANGE,
    REFERER,
    REFERRER_POLICY,
    RETRY_AFTER,
    SERVER,
    SET_COOKIE,
    STRICT_TRANSPORT_SECURITY,
    TE,
    TRAILER,
    TRANSFER_ENCODING,
    USER_AGENT,
    UPGRADE,
    UPGRADE_INSECURE_REQUESTS,
    VARY,
    VIA,
    WARNING,
    WWW_AUTHENTICATE,
    X_CONTENT_TYPE_OPTIONS,
    X_DNS_PREFETCH_CONTROL,
    X_FRAME_OPTIONS,
    X_XSS_PROTECTION,
];

#[test_case]
fn get_invalid() {
    let mut headers = HeaderMap::new();
    headers.insert("foo", "bar".parse().unwrap());
    assert!(headers.get("Evil\r\nKey").is_none());
}

//#[test_case]
//#[should_panic]
fn insert_invalid() {
    let mut headers = HeaderMap::new();
    headers.insert("evil\r\nfoo", "bar".parse().unwrap());
}

#[test_case]
fn value_htab() {
    // RFC 7230 Section 3.2:
    // > field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
    HeaderValue::from_static("hello\tworld");
    HeaderValue::from_str("hello\tworld").unwrap();
}



pub fn run_tests() {
    run_inventory_tests!();
}

#[no_mangle]
pub extern "C" fn ecall_run_tests() {
    run_tests();
}
