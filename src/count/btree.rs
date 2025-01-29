use std::collections::BTreeMap;
use std::io;

/// Default limit(intentionally low value)
pub const UNIQUE_ELEMENT_COUNT_LIMIT_DEFAULT: usize = 16;

pub fn strings2count_limited<I>(
    strings: I,
    limit: usize,
) -> Result<BTreeMap<String, u32>, io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let mut ret: BTreeMap<String, u32> = BTreeMap::new();

    for rstr in strings {
        let item: String = rstr?;
        let oval: Option<&mut u32> = ret.get_mut(&item);
        match oval {
            None => {
                let sz: usize = ret.len();
                if limit < sz {
                    return Err(io::Error::other(format!(
						"too many elements({limit}). make it bigger using ENV_UNIQUE_ELEMENT_LIMIT."
					)));
                }
                ret.insert(item, 1);
            }
            Some(mu) => *mu += 1,
        }
    }

    Ok(ret)
}

pub fn strings2count_limited_default<I>(strings: I) -> Result<BTreeMap<String, u32>, io::Error>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    strings2count_limited(strings, UNIQUE_ELEMENT_COUNT_LIMIT_DEFAULT)
}
