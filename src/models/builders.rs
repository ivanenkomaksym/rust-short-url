use super::linkinfo::LinkInfo;

pub(crate) fn build_link_info(short_url: String, long_url: String) -> LinkInfo {
    let link_info = LinkInfo{
        short_url,
        long_url,
        clicks: 0,
        analytics: Vec::new()
    };

    link_info
}