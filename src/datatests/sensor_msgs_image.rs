//! Perform tests with sensor_msgs/Image

#[cfg(test)]
mod tests {
    use from_slice;

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Image {
        pub header: Header,
        pub height: u32,
        pub width: u32,
        pub encoding: String,
        pub is_bigendian: u8,
        pub step: u32,
        pub data: Vec<u8>,
    }

    // Not supported at the moment
    // #[derive(Deserialize, Serialize, PartialEq, Debug)]
    // pub struct RefImage<'a> {
    //     pub header: Header,
    //     pub height: u32,
    //     pub width: u32,
    //     pub encoding: String,
    //     pub is_bigendian: u8,
    //     pub step: u32,
    //     pub data: &'a [u8],
    // }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct SharedImage {
        pub header: Header,
        pub height: u32,
        pub width: u32,
        pub encoding: String,
        pub is_bigendian: u8,
        pub step: u32,
        pub data: Box<[u8]>,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Header {
        pub seq: u32,
        pub stamp: Time,
        pub frame_id: String,
    }

    #[derive(Deserialize, Serialize, PartialEq, Debug)]
    pub struct Time {
        pub secs: u32,
        pub nsecs: u32,
    }

    #[test]
    fn reads_message() {
        let expectation = Image {
            header: Header {
                stamp: Time { secs: 0, nsecs: 0 },
                frame_id: "test".to_string(),
                seq: 0,
            },
            height: 1080,
            width: 1920,
            encoding: "bgr8".to_string(),
            is_bigendian: false as u8,
            step: 5760,
            data: vec![0; 1080 * 1920 * 3],
        };
        let bytes = include_bytes!("sensor_msgs_image_1080p.bin");
        let msg: Image = from_slice(bytes).unwrap();
        assert_eq!(expectation, msg);
        // Prove that deserialization works with different representations of Image
        // Assume that data is same
        let _msg: SharedImage = from_slice(bytes).unwrap();
        // let _msg: RefImage = from_slice(bytes).unwrap();
    }

}
