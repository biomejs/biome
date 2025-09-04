use std::borrow::Cow;
use std::char::{DecodeUtf16Error, decode_utf16};

pub(crate) fn decode_maybe_utf16_string(
    mut content: &[u8],
) -> Result<Cow<'_, str>, DecodeUtf16Error> {
    enum FileEncoding {
        Unknown,
        Utf8,
        Utf16Le,
        Utf16Be,
    }

    let mut encoding = FileEncoding::Unknown;

    // Read the BOM if present and skip it
    let bom = content.get(0..3);
    if let Some(&[0xef, 0xbb, 0xbf]) = bom {
        content = &content[3..];
        encoding = FileEncoding::Utf8;
    } else if let Some(&[0xfe, 0xff, _]) = bom {
        content = &content[2..];
        encoding = FileEncoding::Utf16Be;
    } else if let Some(&[0xff, 0xfe, _]) = bom {
        content = &content[2..];
        encoding = FileEncoding::Utf16Le;
    }

    if matches!(encoding, FileEncoding::Unknown | FileEncoding::Utf8) {
        // Attempt to parse as UTF-8
        let result = std::str::from_utf8(content);

        if let FileEncoding::Utf8 = encoding {
            // If the file is known to be UTF-8 unwrap the result
            return Ok(Cow::Borrowed(result.unwrap()));
        } else if let Ok(result) = result {
            // Otherwise, only return if the parsing was successful
            return Ok(Cow::Borrowed(result));
        }
    }

    // If a UTF-16 BOM was found or an error was encountered, attempt to parse as UTF-16
    let content_str = decode_utf16(content.chunks(2).map(|bytes| match encoding {
        FileEncoding::Utf16Be => u16::from_be_bytes([bytes[0], bytes[1]]),
        FileEncoding::Utf16Le => u16::from_le_bytes([bytes[0], bytes[1]]),
        // If the encoding is unknown attempt to decode in native endianness
        FileEncoding::Unknown => u16::from_ne_bytes([bytes[0], bytes[1]]),
        FileEncoding::Utf8 => unreachable!(),
    }))
    .collect::<Result<String, _>>()?;

    Ok(Cow::Owned(content_str))
}
