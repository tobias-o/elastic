//! http://www.elastic.co/guide/en/elasticsearch/reference/master/indices-types-exists.html

//Autogenerated

use hyper::client::Client;
use hyper::client::response::Response;
use hyper::error::Result;

use RequestParams;

pub fn head_index_type<'a>(client: &'a mut Client, req: RequestParams,
                       index: &'a str, _type: &'a str) -> Result<Response>{
    let url_qry = &req.get_url_qry();
    let base = &req.base_url;
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + index.len() + _type.len() +
                                  url_qry.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str(url_qry);
    let res = client.head(&url_fmtd).headers(req.headers);
    res.send()
}
