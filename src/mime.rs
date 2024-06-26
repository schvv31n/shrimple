use crate::short_str;
use crate::utils::ShortStr;
use anyhow::{bail, Context, Result};
use shrimple_parser::tuple::first;
use ureq::Response;

macro_rules! define_mime_to_ext {
    {$( $variant_str:literal => $ext:literal ),+ $(,)?} => {
        pub fn mime_to_ext(src: &str) -> Result<ShortStr> {
            match src.split_once(['+', ';']).map_or(src, first) {
                $( $variant_str => Ok(short_str!($ext)), )+
                _ => bail!("unknown MIME type: {src}"),
            }
        }
    };
}

define_mime_to_ext! {
    "text/h323" => ".323",
    "application/octet-stream" => "",
    "text/plain" => ".txt",
    "application/msaccess" => ".accdb",
    "application/internet-property-stream" => ".acx",
    "audio/x-aiff" => ".aif",
    "audio/aiff" => ".aiff",
    "application/x-ms-application" => ".application",
    "image/x-jg" => ".art",
    "video/x-ms-asf" => ".asf",
    "application/atom" => ".atom",
    "audio/basic" => ".au",
    "video/x-msvideo" => ".avi",
    "application/olescript" => ".axs",
    "application/x-bcpio" => ".bcpio",
    "image/bmp" => ".bmp",
    "application/vnd.ms-office.calx" => ".calx",
    "application/vnd.ms-pki.seccat" => ".cat",
    "application/x-cdf" => ".cdf",
    "application/x-java-applet" => ".class",
    "application/x-msclip" => ".clp",
    "image/x-cmx" => ".cmx",
    "image/cis-cod" => ".cod",
    "application/x-cpio" => ".cpio",
    "application/x-mscardfile" => ".crd",
    "application/pkix-crl" => ".crl",
    "application/x-x509-ca-cert" => ".crt",
    "application/x-csh" => ".csh",
    "text/css" => ".css",
    "application/x-director" => ".dir",
    "application/x-msdownload" => ".dll",
    "text/dlm" => ".dlm",
    "application/msword" => ".doc",
    "application/vnd.ms-word.document.macroEnabled.12" => ".docm",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => ".docx",
    "application/vnd.ms-word.template.macroEnabled.12" => ".dotm",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.template" => ".dotx",
    "application/x-dvi" => ".dvi",
    "drawing/x-dwf" => ".dwf",
    "text/x-setext" => ".etx",
    "application/envoy" => ".evy",
    "application/vnd.fdf" => ".fdf",
    "application/fractals" => ".fif",
    "x-world/x-vrml" => ".flr",
    "video/x-flv" => ".flv",
    "image/gif" => ".gif",
    "application/x-gtar" => ".gtar",
    "application/x-gzip" => ".gz",
    "application/x-hdf" => ".hdf",
    "text/x-hdml" => ".hdml",
    "application/x-oleobject" => ".hhc",
    "application/winhlp" => ".hlp",
    "application/mac-binhex40" => ".hqx",
    "application/hta" => ".hta",
    "text/x-component" => ".htc",
    "text/html" => ".html",
    "text/webviewhtml" => ".htt",
    "image/x-icon" => ".ico",
    "image/ief" => ".ief",
    "application/x-iphone" => ".iii",
    "application/x-internet-signup" => ".ins",
    "video/x-ivf" => ".IVF",
    "application/java-archive" => ".jar",
    "application/liquidmotion" => ".jck",
    "image/pjpeg" => ".jfif",
    "image/jpeg" => ".jpeg",
    "application/x-javascript" => ".js",
    "text/jscript" => ".jsx",
    "application/x-latex" => ".latex",
    "application/x-ms-reader" => ".lit",
    "video/x-la-asf" => ".lsf",
    "application/x-msmediaview" => ".m13",
    "audio/x-mpegurl" => ".m3u",
    "application/x-troff-man" => ".man",
    "application/x-ms-manifest" => ".manifest",
    "application/x-msaccess" => ".mdb",
    "application/x-troff-me" => ".me",
    "message/rfc822" => ".mhtml",
    "audio/mid" => ".midi",
    "application/x-smaf" => ".mmf",
    "application/x-msmoney" => ".mny",
    "video/quicktime" => ".mov",
    "video/x-sgi-movie" => ".movie",
    "audio/mpeg" => ".mp3",
    "video/mpeg" => ".mpeg",
    "application/vnd.ms-project" => ".mpp",
    "application/x-troff-ms" => ".ms",
    "application/x-miva-compiled" => ".mvc",
    "application/x-netcdf" => ".nc",
    "application/oda" => ".oda",
    "text/x-ms-odc" => ".odc",
    "application/oleobject" => ".ods",
    "application/onenote" => ".one",
    "application/opensearchdescription" => ".osdx",
    "application/pkcs10" => ".p10",
    "application/x-pkcs12" => ".p12",
    "application/x-pkcs7-certificates" => ".p7b",
    "application/pkcs7-mime" => ".p7c",
    "application/x-pkcs7-certreqresp" => ".p7r",
    "application/pkcs7-signature" => ".p7s",
    "image/x-portable-bitmap" => ".pbm",
    "application/pdf" => ".pdf",
    "image/x-portable-graymap" => ".pgm",
    "application/vnd.ms-pki.pko" => ".pko",
    "application/x-perfmon" => ".pma",
    "image/png" => ".png",
    "image/x-portable-anymap" => ".pnm",
    "application/vnd.ms-powerpoint" => ".pot",
    "application/vnd.ms-powerpoint.template.macroEnabled.12" => ".potm",
    "application/vnd.openxmlformats-officedocument.presentationml.template" => ".potx",
    "application/vnd.ms-powerpoint.addin.macroEnabled.12" => ".ppam",
    "image/x-portable-pixmap" => ".ppm",
    "application/vnd.ms-powerpoint.slideshow.macroEnabled.12" => ".ppsm",
    "application/vnd.openxmlformats-officedocument.presentationml.slideshow" => ".ppsx",
    "application/vnd.ms-powerpoint.presentation.macroEnabled.12" => ".pptm",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation" => ".pptx",
    "application/pics-rules" => ".prf",
    "application/postscript" => ".ps",
    "application/x-mspublisher" => ".pub",
    "application/x-quicktimeplayer" => ".qtl",
    "audio/x-pn-realaudio" => ".ra",
    "image/x-cmu-raster" => ".ras",
    "image/vnd.rn-realflash" => ".rf",
    "image/x-rgb" => ".rgb",
    "application/vnd.rn-realmedia" => ".rm",
    "application/x-troff" => ".roff",
    "audio/x-pn-realaudio-plugin" => ".rpm",
    "application/rtf" => ".rtf",
    "text/richtext" => ".rtx",
    "application/x-msschedule" => ".scd",
    "text/scriptlet" => ".sct",
    "application/set-payment-initiation" => ".setpay",
    "application/set-registration-initiation" => ".setreg",
    "text/sgml" => ".sgml",
    "application/x-sh" => ".sh",
    "application/x-shar" => ".shar",
    "application/x-stuffit" => ".sit",
    "application/vnd.ms-powerpoint.slide.macroEnabled.12" => ".sldm",
    "application/vnd.openxmlformats-officedocument.presentationml.slide" => ".sldx",
    "audio/x-smd" => ".smd",
    "application/futuresplash" => ".spl",
    "application/x-wais-source" => ".src",
    "application/streamingmedia" => ".ssm",
    "application/vnd.ms-pki.certstore" => ".sst",
    "application/vnd.ms-pki.stl" => ".stl",
    "application/x-sv4cpio" => ".sv4cpio",
    "application/x-sv4crc" => ".sv4crc",
    "image/svg" => ".svg",
    "application/x-shockwave-flash" => ".swf",
    "application/x-tar" => ".tar",
    "application/x-tcl" => ".tcl",
    "application/x-tex" => ".tex",
    "application/x-texinfo" => ".texi",
    "application/x-compressed" => ".tgz",
    "application/vnd.ms-officetheme" => ".thmx",
    "image/tiff" => ".tiff",
    "application/x-msterminal" => ".trm",
    "text/tab-separated-values" => ".tsv",
    "text/iuls" => ".uls",
    "application/x-ustar" => ".ustar",
    "text/vbscript" => ".vbs",
    "text/x-vcard" => ".vcf",
    "application/vnd.ms-visio.viewer" => ".vdx",
    "application/vnd.visio" => ".vsd",
    "application/x-ms-vsto" => ".vsto",
    "audio/wav" => ".wav",
    "audio/x-ms-wax" => ".wax",
    "image/vnd.wap.wbmp" => ".wbmp",
    "application/vnd.ms-works" => ".wcm",
    "video/x-ms-wm" => ".wm",
    "audio/x-ms-wma" => ".wma",
    "application/x-ms-wmd" => ".wmd",
    "application/x-msmetafile" => ".wmf",
    "text/vnd.wap.wml" => ".wml",
    "application/vnd.wap.wmlc" => ".wmlc",
    "text/vnd.wap.wmlscript" => ".wmls",
    "application/vnd.wap.wmlscriptc" => ".wmlsc",
    "video/x-ms-wmp" => ".wmp",
    "video/x-ms-wmv" => ".wmv",
    "video/x-ms-wmx" => ".wmx",
    "application/x-ms-wmz" => ".wmz",
    "application/x-mswrite" => ".wri",
    "video/x-ms-wvx" => ".wvx",
    "application/directx" => ".x",
    "application/xaml" => ".xaml",
    "application/x-silverlight-app" => ".xap",
    "application/x-ms-xbap" => ".xbap",
    "image/x-xbitmap" => ".xbm",
    "application/vnd.ms-excel.addin.macroEnabled.12" => ".xlam",
    "application/vnd.ms-excel" => ".xls",
    "application/vnd.ms-excel.sheet.binary.macroEnabled.12" => ".xlsb",
    "application/vnd.ms-excel.sheet.macroEnabled.12" => ".xlsm",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => ".xlsx",
    "application/vnd.ms-excel.template.macroEnabled.12" => ".xltm",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.template" => ".xltx",
    "text/xml" => ".xml",
    "image/x-xpixmap" => ".xpm",
    "application/vnd.ms-xpsdocument" => ".xps",
    "image/x-xwindowdump" => ".xwd",
    "application/x-compress" => ".z",
    "application/x-zip-compressed" => ".zip",
}

/// Returns the file extension most suitable for the content of the HTTP response.
pub fn remote_file_ext(response: &Response) -> Result<ShortStr> {
    let path = response.get_url();
    if let Some(last_dot) = path.rfind('.') {
        let ext = path.split_at(last_dot).1;
        ShortStr::new(ext).with_context(|| format!("file extension too long: {ext:?}"))
    } else {
        let content_type =
            response.header("Content-Type").context("no Content-Type header provided")?;
        mime_to_ext(content_type)
    }
}
