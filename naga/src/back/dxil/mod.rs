const DXIL_CONTAINER_HASH_SIZE: usize = 16;
/// Current major version.
const DXIL_CONTAINER_VERSION_MAJOR: u16 = 1;
/// Current minor version.
const DXIL_CONTAINER_VERSION_MINOR: u16 = 0;
/// Max size for container.
const DXIL_CONTAINER_MAX_SIZE: u32 = 0x80000000;

/// Use this type to represent the hash for the full container.
#[repr(C)]
struct DxilContainerHash {
    digest: [u8; DXIL_CONTAINER_HASH_SIZE],
}

#[repr(C)]
struct DxilContainerVersion {
    /// Must be [`DXIL_CONTAINER_VERSION_MAJOR`].
    major: u16,
    /// Must be [`DXIL_CONTAINER_VERSION_MINOR`].
    minor: u16,
}

/// Use this type to describe a DXIL container of parts.
#[repr(C)]
struct DxilContainerHeader {
    /// Must be [`CONTAINER_FOUR_CC`].
    header_four_cc: u32,
    hash: DxilContainerHash,
    version: DxilContainerVersion,
    /// From start of this header.
    container_size_in_bytes: u32,
    part_count: u32,
    // Structure is followed by uint32_t PartOffset[PartCount];
    // The offset is to a DxilPartHeader.
}
/// for back-compat with tools that look for DXBC containers.
const CONTAINER_FOUR_CC: u32 = u32::from_le_bytes(*b"DXBC");

#[repr(u32)]
enum DxilFourCC {
    ResourceDef = u32::from_le_bytes(*b"RDEF"),
    InputSignature = u32::from_le_bytes(*b"ISG1"),
    OutputSignature = u32::from_le_bytes(*b"OSG1"),
    PatchConstantSignature = u32::from_le_bytes(*b"PSG1"),
    ShaderStatistics = u32::from_le_bytes(*b"STAT"),
    /// This is an LLVM module with debug information. It's an augmented version of the original DXIL module.
    /// For historical reasons, this is sometimes referred to as 'the PDB of the program'.
    ShaderDebugInfoDXIL = u32::from_le_bytes(*b"ILDB"),
    /// This is a name for an external entity holding the debug information.
    ShaderDebugName = u32::from_le_bytes(*b"ILDN"),
    FeatureInfo = u32::from_le_bytes(*b"SFI0"),
    PrivateData = u32::from_le_bytes(*b"PRIV"),
    RootSignature = u32::from_le_bytes(*b"RTS0"),
    /// A valid DXIL program has no debug information. This is the program described by debug information.
    Dxil = u32::from_le_bytes(*b"DXIL"),
    PipelineStateValidation = u32::from_le_bytes(*b"PSV0"),
    RuntimeData = u32::from_le_bytes(*b"RDAT"),
    ShaderHash = u32::from_le_bytes(*b"HASH"),
    ShaderSourceInfo = u32::from_le_bytes(*b"SRCI"),
    ShaderPDBInfo = u32::from_le_bytes(*b"PDBI"),
    CompilerVersion = u32::from_le_bytes(*b"VERS"),
}

/// Use this type to describe the size and type of a DXIL container part.
#[repr(C)]
struct DxilPartHeader {
    /// Four char code for part type.
    part_four_cc: DxilFourCC,
    /// Byte count for PartData.
    part_size: u32,
    // Structure is followed by uint8_t PartData[PartSize].
}

/// DXIL program information.
#[repr(C)]
struct DxilBitcodeHeader {
    /// Must be [`DXIL_MAGIC_VALUE`].
    dxil_magic: u32,
    /// DXIL version.
    dxil_version: u32,
    /// Offset to LLVM bitcode (from start of header).
    bitcode_offset: u32,
    /// Size of LLVM bitcode.
    bitcode_size: u32,
}
const DXIL_MAGIC_VALUE: u32 = u32::from_le_bytes(*b"DXIL");

#[repr(C)]
struct DxilProgramHeader {
    /// Major and minor version, including type.
    program_version: u32,
    /// Size in uint32_t units including this header.
    size_in_uint32: u32,
    /// Bitcode-specific header.
    bitcode_header: DxilBitcodeHeader,
    // Followed by uint8_t[BitcodeHeader.BitcodeOffset]
}

/// Extract the shader type from the program version value.
// inline DXIL::ShaderKind GetVersionShaderType(uint32_t programVersion) {
//     return (DXIL::ShaderKind)((programVersion & 0xffff0000) >> 16);
//   }
//   inline uint32_t GetVersionMajor(uint32_t programVersion) {
//     return (programVersion & 0xf0) >> 4;
//   }
//   inline uint32_t GetVersionMinor(uint32_t programVersion) {
//     return (programVersion & 0xf);
//   }
//   inline uint32_t EncodeVersion(DXIL::ShaderKind shaderType, uint32_t major,
//                                 uint32_t minor) {
//     return ((unsigned)shaderType << 16) | (major << 4) | minor;
//   }
