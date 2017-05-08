use std::io::Error as IOError;
use typedef::NiftiType;

quick_error! {
    #[derive(Debug)]
    pub enum NiftiError {
        /// An invalid NIfTI-1 file was parsed.
        /// This is detected when reading the file's magic code,
        /// which should be either `b"ni1\0"` or `b"n+1\0`.
        InvalidFormat {
            description("Invalid NIfTI-1 file")
        }
        /// Attempted to read volume outside boundaries.
        OutOfBounds {
            description("Out of bounds access to volume")
        }
        /// An attempt to read a complete NIFTI-1 object was attempted.
        /// Can be triggered when a NIFTI object contains the magic code
        /// "ni-1\0", even if the following bytes contain the volume.
        NoVolumeData {
            description("No volume data available")
        }
        IncorrectVolumeDimensionality(expected: u16, got: u16) {
            description("Unexpected volume data dimensionality")
        }
        /// This voxel data type is not supported. Sorry. :(
        UnsupportedDataType(t: NiftiType) {
            description("Unsupported data type")
        }
        /// I/O Error
        Io(err: IOError) {
            from()
            cause(err)
            description(err.description())
        }

    }
}

pub type Result<T> = ::std::result::Result<T, NiftiError>;