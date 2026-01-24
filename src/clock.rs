use time::OffsetDateTime;
use time::format_description;

pub(crate) fn get() -> String {
    let format = format_description::parse(
        "[weekday], [month repr:short] [day padding:zero] > [hour repr:24 padding:zero]:[minute padding:zero]"
    ).unwrap();
    
    OffsetDateTime::now_local()
        .unwrap()
        .format(&format)
        .unwrap()
}
