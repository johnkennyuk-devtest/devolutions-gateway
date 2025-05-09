use webm_iterable::matroska_spec::{Master, MatroskaSpec};

pub fn mastroka_spec_name(spec: &MatroskaSpec) -> &'static str {
    match spec {
        MatroskaSpec::Ebml(Master::Start) => "Ebml Start",
        MatroskaSpec::Ebml(Master::End) => "Ebml End",
        MatroskaSpec::Ebml(Master::Full(_)) => "Ebml Full",
        MatroskaSpec::EbmlVersion(_) => "Ebml Version",
        MatroskaSpec::EbmlReadVersion(_) => "Ebml Read Version",
        MatroskaSpec::EbmlMaxIdLength(_) => "Ebml Max Id Length",
        MatroskaSpec::EbmlMaxSizeLength(_) => "Ebml Max Size Length",
        MatroskaSpec::DocType(_) => "Doc Type",
        MatroskaSpec::DocTypeVersion(_) => "Doc Type Version",
        MatroskaSpec::DocTypeReadVersion(_) => "Doc Type Read Version",
        MatroskaSpec::DocTypeExtension(Master::Start) => "DocType Extension Start",
        MatroskaSpec::DocTypeExtension(Master::End) => "DocType Extension End",
        MatroskaSpec::DocTypeExtension(Master::Full(_)) => "DocType Extension Full",
        MatroskaSpec::DocTypeExtensionName(_) => "Doc Type Extension Name",
        MatroskaSpec::DocTypeExtensionVersion(_) => "Doc Type Extension Version",
        MatroskaSpec::Segment(Master::Start) => "Segment Start",
        MatroskaSpec::Segment(Master::End) => "Segment End",
        MatroskaSpec::Segment(Master::Full(_)) => "Segment Full",
        MatroskaSpec::Attachments(Master::Start) => "Attachments Start",
        MatroskaSpec::Attachments(Master::End) => "Attachments End",
        MatroskaSpec::Attachments(Master::Full(_)) => "Attachments Full",
        MatroskaSpec::AttachedFile(Master::Start) => "Attached File Start",
        MatroskaSpec::AttachedFile(Master::End) => "Attached File End",
        MatroskaSpec::AttachedFile(Master::Full(_)) => "Attached File Full",
        MatroskaSpec::FileData(_) => "File Data",
        MatroskaSpec::FileDescription(_) => "File Description",
        MatroskaSpec::FileMimeType(_) => "File Mime Type",
        MatroskaSpec::FileName(_) => "File Name",
        MatroskaSpec::FileReferral(_) => "File Referral",
        MatroskaSpec::FileUID(_) => "File UID",
        MatroskaSpec::FileUsedEndTime(_) => "File Used End Time",
        MatroskaSpec::FileUsedStartTime(_) => "File Used Start Time",
        MatroskaSpec::Chapters(Master::Start) => "Chapters Start",
        MatroskaSpec::Chapters(Master::End) => "Chapters End",
        MatroskaSpec::Chapters(Master::Full(_)) => "Chapters Full",
        MatroskaSpec::EditionEntry(Master::Start) => "Edition Entry Start",
        MatroskaSpec::EditionEntry(Master::End) => "Edition Entry End",
        MatroskaSpec::EditionEntry(Master::Full(_)) => "Edition Entry Full",
        MatroskaSpec::ChapterAtom(Master::Start) => "Chapter Atom Start",
        MatroskaSpec::ChapterAtom(Master::End) => "Chapter Atom End",
        MatroskaSpec::ChapterAtom(Master::Full(_)) => "Chapter Atom Full",
        MatroskaSpec::ChapProcess(Master::Start) => "Chap Process Start",
        MatroskaSpec::ChapProcess(Master::End) => "Chap Process End",
        MatroskaSpec::ChapProcess(Master::Full(_)) => "Chap Process Full",
        MatroskaSpec::ChapProcessCodecID(_) => "Chap Process Codec ID",
        MatroskaSpec::ChapProcessCommand(Master::Start) => "Chap Process Command Start",
        MatroskaSpec::ChapProcessCommand(Master::End) => "Chap Process Command End",
        MatroskaSpec::ChapProcessCommand(Master::Full(_)) => "Chap Process Command Full",
        MatroskaSpec::ChapProcessData(_) => "Chap Process Data",
        MatroskaSpec::ChapProcessTime(_) => "Chap Process Time",
        MatroskaSpec::ChapProcessPrivate(_) => "Chap Process Private",
        MatroskaSpec::ChapterDisplay(Master::Start) => "Chapter Display Start",
        MatroskaSpec::ChapterDisplay(Master::End) => "Chapter Display End",
        MatroskaSpec::ChapterDisplay(Master::Full(_)) => "Chapter Display Full",
        MatroskaSpec::ChapCountry(_) => "Chap Country",
        MatroskaSpec::ChapLanguage(_) => "Chap Language",
        MatroskaSpec::ChapLanguageIETF(_) => "Chap Language IETF",
        MatroskaSpec::ChapString(_) => "Chap String",
        MatroskaSpec::ChapterFlagEnabled(_) => "Chapter Flag Enabled",
        MatroskaSpec::ChapterFlagHidden(_) => "Chapter Flag Hidden",
        MatroskaSpec::ChapterPhysicalEquiv(_) => "Chapter Physical Equiv",
        MatroskaSpec::ChapterSegmentEditionUID(_) => "Chapter Segment Edition UID",
        MatroskaSpec::ChapterSegmentUID(_) => "Chapter Segment UID",
        MatroskaSpec::ChapterStringUID(_) => "Chapter String UID",
        MatroskaSpec::ChapterTimeEnd(_) => "Chapter Time End",
        MatroskaSpec::ChapterTimeStart(_) => "Chapter Time Start",
        MatroskaSpec::ChapterTrack(Master::Start) => "Chapter Track Start",
        MatroskaSpec::ChapterTrack(Master::End) => "Chapter Track End",
        MatroskaSpec::ChapterTrack(Master::Full(_)) => "Chapter Track Full",
        MatroskaSpec::ChapterTrackUID(_) => "Chapter Track UID",
        MatroskaSpec::ChapterUID(_) => "Chapter UID",
        MatroskaSpec::EditionFlagDefault(_) => "Edition Flag Default",
        MatroskaSpec::EditionFlagHidden(_) => "Edition Flag Hidden",
        MatroskaSpec::EditionFlagOrdered(_) => "Edition Flag Ordered",
        MatroskaSpec::EditionUID(_) => "Edition UID",
        MatroskaSpec::Cluster(Master::Start) => "Cluster Start",
        MatroskaSpec::Cluster(Master::End) => "Cluster End",
        MatroskaSpec::Cluster(Master::Full(_)) => "Cluster Full",
        MatroskaSpec::BlockGroup(Master::Start) => "Block Group Start",
        MatroskaSpec::BlockGroup(Master::End) => "Block Group End",
        MatroskaSpec::BlockGroup(Master::Full(_)) => "Block Group Full",
        MatroskaSpec::Block(_) => "Block",
        MatroskaSpec::BlockAdditions(Master::Start) => "Block Additions Start",
        MatroskaSpec::BlockAdditions(Master::End) => "Block Additions End",
        MatroskaSpec::BlockAdditions(Master::Full(_)) => "Block Additions Full",
        MatroskaSpec::BlockMore(Master::Start) => "Block More Start",
        MatroskaSpec::BlockMore(Master::End) => "Block More End",
        MatroskaSpec::BlockMore(Master::Full(_)) => "Block More Full",
        MatroskaSpec::BlockAddID(_) => "Block Add ID",
        MatroskaSpec::BlockAdditional(_) => "Block Additional",
        MatroskaSpec::BlockDuration(_) => "Block Duration",
        MatroskaSpec::BlockVirtual(_) => "Block Virtual",
        MatroskaSpec::CodecState(_) => "Codec State",
        MatroskaSpec::DiscardPadding(_) => "Discard Padding",
        MatroskaSpec::ReferenceBlock(_) => "Reference Block",
        MatroskaSpec::ReferenceFrame(Master::Start) => "Reference Frame Start",
        MatroskaSpec::ReferenceFrame(Master::End) => "Reference Frame End",
        MatroskaSpec::ReferenceFrame(Master::Full(_)) => "Reference Frame Full",
        MatroskaSpec::ReferenceOffset(_) => "Reference Offset",
        MatroskaSpec::ReferenceTimestamp(_) => "Reference Timestamp",
        MatroskaSpec::ReferencePriority(_) => "Reference Priority",
        MatroskaSpec::ReferenceVirtual(_) => "Reference Virtual",
        MatroskaSpec::Slices(Master::Start) => "Slices Start",
        MatroskaSpec::Slices(Master::End) => "Slices End",
        MatroskaSpec::Slices(Master::Full(_)) => "Slices Full",
        MatroskaSpec::TimeSlice(Master::Start) => "Time Slice Start",
        MatroskaSpec::TimeSlice(Master::End) => "Time Slice End",
        MatroskaSpec::TimeSlice(Master::Full(_)) => "Time Slice Full",
        MatroskaSpec::BlockAdditionID(_) => "Block Addition ID",
        MatroskaSpec::Delay(_) => "Delay",
        MatroskaSpec::FrameNumber(_) => "Frame Number",
        MatroskaSpec::LaceNumber(_) => "Lace Number",
        MatroskaSpec::SliceDuration(_) => "Slice Duration",
        MatroskaSpec::EncryptedBlock(_) => "Encrypted Block",
        MatroskaSpec::Position(_) => "Position",
        MatroskaSpec::PrevSize(_) => "Prev Size",
        MatroskaSpec::SilentTracks(Master::Start) => "Silent Tracks Start",
        MatroskaSpec::SilentTracks(Master::End) => "Silent Tracks End",
        MatroskaSpec::SilentTracks(Master::Full(_)) => "Silent Tracks Full",
        MatroskaSpec::SilentTrackNumber(_) => "Silent Track Number",
        MatroskaSpec::SimpleBlock(_) => "Simple Block",
        MatroskaSpec::Timestamp(_) => "Timestamp",
        MatroskaSpec::Cues(Master::Start) => "Cues Start",
        MatroskaSpec::Cues(Master::End) => "Cues End",
        MatroskaSpec::Cues(Master::Full(_)) => "Cues Full",
        MatroskaSpec::CuePoint(Master::Start) => "Cue Point Start",
        MatroskaSpec::CuePoint(Master::End) => "Cue Point End",
        MatroskaSpec::CuePoint(Master::Full(_)) => "Cue Point Full",
        MatroskaSpec::CueTime(_) => "Cue Time",
        MatroskaSpec::CueTrackPositions(Master::Start) => "Cue Track Positions Start",
        MatroskaSpec::CueTrackPositions(Master::End) => "Cue Track Positions End",
        MatroskaSpec::CueTrackPositions(Master::Full(_)) => "Cue Track Positions Full",
        MatroskaSpec::CueBlockNumber(_) => "Cue Block Number",
        MatroskaSpec::CueClusterPosition(_) => "Cue Cluster Position",
        MatroskaSpec::CueCodecState(_) => "Cue Codec State",
        MatroskaSpec::CueDuration(_) => "Cue Duration",
        MatroskaSpec::CueReference(Master::Start) => "Cue Reference Start",
        MatroskaSpec::CueReference(Master::End) => "Cue Reference End",
        MatroskaSpec::CueReference(Master::Full(_)) => "Cue Reference Full",
        MatroskaSpec::CueRefCluster(_) => "Cue Ref Cluster",
        MatroskaSpec::CueRefCodecState(_) => "Cue Ref Codec State",
        MatroskaSpec::CueRefNumber(_) => "Cue Ref Number",
        MatroskaSpec::CueRefTime(_) => "Cue Ref Time",
        MatroskaSpec::CueRelativePosition(_) => "Cue Relative Position",
        MatroskaSpec::CueTrack(_) => "Cue Track",
        MatroskaSpec::Info(Master::Start) => "Info Start",
        MatroskaSpec::Info(Master::End) => "Info End",
        MatroskaSpec::Info(Master::Full(_)) => "Info Full",
        MatroskaSpec::ChapterTranslate(Master::Start) => "Chapter Translate Start",
        MatroskaSpec::ChapterTranslate(Master::End) => "Chapter Translate End",
        MatroskaSpec::ChapterTranslate(Master::Full(_)) => "Chapter Translate Full",
        MatroskaSpec::ChapterTranslateCodec(_) => "Chapter Translate Codec",
        MatroskaSpec::ChapterTranslateEditionUID(_) => "Chapter Translate Edition UID",
        MatroskaSpec::ChapterTranslateID(_) => "Chapter Translate ID",
        MatroskaSpec::DateUTC(_) => "Date UTC",
        MatroskaSpec::Duration(_) => "Duration",
        MatroskaSpec::MuxingApp(_) => "Muxing App",
        MatroskaSpec::NextFilename(_) => "Next Filename",
        MatroskaSpec::NextUID(_) => "Next UID",
        MatroskaSpec::PrevFilename(_) => "Prev Filename",
        MatroskaSpec::PrevUID(_) => "Prev UID",
        MatroskaSpec::SegmentFamily(_) => "Segment Family",
        MatroskaSpec::SegmentFilename(_) => "Segment Filename",
        MatroskaSpec::SegmentUID(_) => "Segment UID",
        MatroskaSpec::TimestampScale(_) => "Timestamp Scale",
        MatroskaSpec::Title(_) => "Title",
        MatroskaSpec::WritingApp(_) => "Writing App",
        MatroskaSpec::SeekHead(Master::Start) => "Seek Head Start",
        MatroskaSpec::SeekHead(Master::End) => "Seek Head End",
        MatroskaSpec::SeekHead(Master::Full(_)) => "Seek Head Full",
        MatroskaSpec::Seek(Master::Start) => "Seek Start",
        MatroskaSpec::Seek(Master::End) => "Seek End",
        MatroskaSpec::Seek(Master::Full(_)) => "Seek Full",
        MatroskaSpec::SeekID(_) => "Seek ID",
        MatroskaSpec::SeekPosition(_) => "Seek Position",
        MatroskaSpec::Tags(Master::Start) => "Tags Start",
        MatroskaSpec::Tags(Master::End) => "Tags End",
        MatroskaSpec::Tags(Master::Full(_)) => "Tags Full",
        MatroskaSpec::Tag(Master::Start) => "Tag Start",
        MatroskaSpec::Tag(Master::End) => "Tag End",
        MatroskaSpec::Tag(Master::Full(_)) => "Tag Full",
        MatroskaSpec::SimpleTag(Master::Start) => "Simple Tag Start",
        MatroskaSpec::SimpleTag(Master::End) => "Simple Tag End",
        MatroskaSpec::SimpleTag(Master::Full(_)) => "Simple Tag Full",
        MatroskaSpec::TagBinary(_) => "Tag Binary",
        MatroskaSpec::TagDefault(_) => "Tag Default",
        MatroskaSpec::TagDefaultBogus(_) => "Tag Default Bogus",
        MatroskaSpec::TagLanguage(_) => "Tag Language",
        MatroskaSpec::TagLanguageIETF(_) => "Tag Language IETF",
        MatroskaSpec::TagName(_) => "Tag Name",
        MatroskaSpec::TagString(_) => "Tag String",
        MatroskaSpec::Targets(Master::Start) => "Targets Start",
        MatroskaSpec::Targets(Master::End) => "Targets End",
        MatroskaSpec::Targets(Master::Full(_)) => "Targets Full",
        MatroskaSpec::TagAttachmentUID(_) => "Tag Attachment UID",
        MatroskaSpec::TagChapterUID(_) => "Tag Chapter UID",
        MatroskaSpec::TagEditionUID(_) => "Tag Edition UID",
        MatroskaSpec::TagTrackUID(_) => "Tag Track UID",
        MatroskaSpec::TargetType(_) => "Target Type",
        MatroskaSpec::TargetTypeValue(_) => "Target Type Value",
        MatroskaSpec::Tracks(Master::Start) => "Tracks Start",
        MatroskaSpec::Tracks(Master::End) => "Tracks End",
        MatroskaSpec::Tracks(Master::Full(_)) => "Tracks Full",
        MatroskaSpec::TrackEntry(Master::Start) => "Track Entry Start",
        MatroskaSpec::TrackEntry(Master::End) => "Track Entry End",
        MatroskaSpec::TrackEntry(Master::Full(_)) => "Track Entry Full",
        MatroskaSpec::AttachmentLink(_) => "Attachment Link",
        MatroskaSpec::Audio(Master::Start) => "Audio Start",
        MatroskaSpec::Audio(Master::End) => "Audio End",
        MatroskaSpec::Audio(Master::Full(_)) => "Audio Full",
        MatroskaSpec::BitDepth(_) => "Bit Depth",
        MatroskaSpec::ChannelPositions(_) => "Channel Positions",
        MatroskaSpec::Channels(_) => "Channels",
        MatroskaSpec::OutputSamplingFrequency(_) => "Output Sampling Frequency",
        MatroskaSpec::SamplingFrequency(_) => "Sampling Frequency",
        MatroskaSpec::BlockAdditionMapping(Master::Start) => "Block Addition Mapping Start",
        MatroskaSpec::BlockAdditionMapping(Master::End) => "Block Addition Mapping End",
        MatroskaSpec::BlockAdditionMapping(Master::Full(_)) => "Block Addition Mapping Full",
        MatroskaSpec::BlockAddIDExtraData(_) => "Block Add ID Extra Data",
        MatroskaSpec::BlockAddIDName(_) => "Block Add ID Name",
        MatroskaSpec::BlockAddIDType(_) => "Block Add ID Type",
        MatroskaSpec::BlockAddIDValue(_) => "Block Add ID Value",
        MatroskaSpec::CodecDecodeAll(_) => "Codec Decode All",
        MatroskaSpec::CodecDelay(_) => "Codec Delay",
        MatroskaSpec::CodecDownloadURL(_) => "Codec Download URL",
        MatroskaSpec::CodecID(_) => "Codec ID",
        MatroskaSpec::CodecInfoURL(_) => "Codec Info URL",
        MatroskaSpec::CodecName(_) => "Codec Name",
        MatroskaSpec::CodecPrivate(_) => "Codec Private",
        MatroskaSpec::CodecSettings(_) => "Codec Settings",
        MatroskaSpec::ContentEncodings(Master::Start) => "Content Encodings Start",
        MatroskaSpec::ContentEncodings(Master::End) => "Content Encodings End",
        MatroskaSpec::ContentEncodings(Master::Full(_)) => "Content Encodings Full",
        MatroskaSpec::ContentEncoding(Master::Start) => "Content Encoding Start",
        MatroskaSpec::ContentEncoding(Master::End) => "Content Encoding End",
        MatroskaSpec::ContentEncoding(Master::Full(_)) => "Content Encoding Full",
        MatroskaSpec::ContentCompression(Master::Start) => "Content Compression Start",
        MatroskaSpec::ContentCompression(Master::End) => "Content Compression End",
        MatroskaSpec::ContentCompression(Master::Full(_)) => "Content Compression Full",
        MatroskaSpec::ContentCompAlgo(_) => "Content Comp Algo",
        MatroskaSpec::ContentCompSettings(_) => "Content Comp Settings",
        MatroskaSpec::ContentEncodingOrder(_) => "Content Encoding Order",
        MatroskaSpec::ContentEncodingScope(_) => "Content Encoding Scope",
        MatroskaSpec::ContentEncodingType(_) => "Content Encoding Type",
        MatroskaSpec::ContentEncryption(Master::Start) => "Content Encryption Start",
        MatroskaSpec::ContentEncryption(Master::End) => "Content Encryption End",
        MatroskaSpec::ContentEncryption(Master::Full(_)) => "Content Encryption Full",
        MatroskaSpec::ContentEncAESSettings(Master::Start) => "Content Enc AES Settings Start",
        MatroskaSpec::ContentEncAESSettings(Master::End) => "Content Enc AES Settings End",
        MatroskaSpec::ContentEncAESSettings(Master::Full(_)) => "Content Enc AES Settings Full",
        MatroskaSpec::AESSettingsCipherMode(_) => "AES Settings Cipher Mode",
        MatroskaSpec::ContentEncAlgo(_) => "Content Enc Algo",
        MatroskaSpec::ContentEncKeyID(_) => "Content Enc Key ID",
        MatroskaSpec::ContentSigAlgo(_) => "Content Sig Algo",
        MatroskaSpec::ContentSigHashAlgo(_) => "Content Sig Hash Algo",
        MatroskaSpec::ContentSigKeyID(_) => "Content Sig Key ID",
        MatroskaSpec::ContentSignature(_) => "Content Signature",
        MatroskaSpec::DefaultDecodedFieldDuration(_) => "Default Decoded Field Duration",
        MatroskaSpec::DefaultDuration(_) => "Default Duration",
        MatroskaSpec::FlagCommentary(_) => "Flag Commentary",
        MatroskaSpec::FlagDefault(_) => "Flag Default",
        MatroskaSpec::FlagEnabled(_) => "Flag Enabled",
        MatroskaSpec::FlagForced(_) => "Flag Forced",
        MatroskaSpec::FlagHearingImpaired(_) => "Flag Hearing Impaired",
        MatroskaSpec::FlagLacing(_) => "Flag Lacing",
        MatroskaSpec::FlagOriginal(_) => "Flag Original",
        MatroskaSpec::FlagTextDescriptions(_) => "Flag Text Descriptions",
        MatroskaSpec::FlagVisualImpaired(_) => "Flag Visual Impaired",
        MatroskaSpec::Language(_) => "Language",
        MatroskaSpec::LanguageIETF(_) => "Language IETF",
        MatroskaSpec::MaxBlockAdditionID(_) => "Max Block Addition ID",
        MatroskaSpec::MaxCache(_) => "Max Cache",
        MatroskaSpec::MinCache(_) => "Min Cache",
        MatroskaSpec::Name(_) => "Name",
        MatroskaSpec::SeekPreRoll(_) => "Seek Pre Roll",
        MatroskaSpec::TrackNumber(_) => "Track Number",
        MatroskaSpec::TrackOffset(_) => "Track Offset",
        MatroskaSpec::TrackOperation(Master::Start) => "Track Operation Start",
        MatroskaSpec::TrackOperation(Master::End) => "Track Operation End",
        MatroskaSpec::TrackOperation(Master::Full(_)) => "Track Operation Full",
        MatroskaSpec::TrackCombinePlanes(Master::Start) => "Track Combine Planes Start",
        MatroskaSpec::TrackCombinePlanes(Master::End) => "Track Combine Planes End",
        MatroskaSpec::TrackCombinePlanes(Master::Full(_)) => "Track Combine Planes Full",
        MatroskaSpec::TrackPlane(Master::Start) => "Track Plane Start",
        MatroskaSpec::TrackPlane(Master::End) => "Track Plane End",
        MatroskaSpec::TrackPlane(Master::Full(_)) => "Track Plane Full",
        MatroskaSpec::TrackPlaneType(_) => "Track Plane Type",
        MatroskaSpec::TrackPlaneUID(_) => "Track Plane UID",
        MatroskaSpec::TrackJoinBlocks(Master::Start) => "Track Join Blocks Start",
        MatroskaSpec::TrackJoinBlocks(Master::End) => "Track Join Blocks End",
        MatroskaSpec::TrackJoinBlocks(Master::Full(_)) => "Track Join Blocks Full",
        MatroskaSpec::TrackJoinUID(_) => "Track Join UID",
        MatroskaSpec::TrackOverlay(_) => "Track Overlay",
        MatroskaSpec::TrackTimestampScale(_) => "Track Timestamp Scale",
        MatroskaSpec::TrackTranslate(Master::Start) => "Track Translate Start",
        MatroskaSpec::TrackTranslate(Master::End) => "Track Translate End",
        MatroskaSpec::TrackTranslate(Master::Full(_)) => "Track Translate Full",
        MatroskaSpec::TrackTranslateCodec(_) => "Track Translate Codec",
        MatroskaSpec::TrackTranslateEditionUID(_) => "Track Translate Edition UID",
        MatroskaSpec::TrackTranslateTrackID(_) => "Track Translate Track ID",
        MatroskaSpec::TrackType(_) => "Track Type",
        MatroskaSpec::TrackUID(_) => "Track UID",
        MatroskaSpec::TrickMasterTrackSegmentUID(_) => "Trick Master Track Segment UID",
        MatroskaSpec::TrickMasterTrackUID(_) => "Trick Master Track UID",
        MatroskaSpec::TrickTrackFlag(_) => "Trick Track Flag",
        MatroskaSpec::TrickTrackSegmentUID(_) => "Trick Track Segment UID",
        MatroskaSpec::TrickTrackUID(_) => "Trick Track UID",
        MatroskaSpec::Video(Master::Start) => "Video Start",
        MatroskaSpec::Video(Master::End) => "Video End",
        MatroskaSpec::Video(Master::Full(_)) => "Video Full",
        MatroskaSpec::AlphaMode(_) => "Alpha Mode",
        MatroskaSpec::AspectRatioType(_) => "Aspect Ratio Type",
        MatroskaSpec::Colour(Master::Start) => "Colour Start",
        MatroskaSpec::Colour(Master::End) => "Colour End",
        MatroskaSpec::Colour(Master::Full(_)) => "Colour Full",
        MatroskaSpec::BitsPerChannel(_) => "Bits Per Channel",
        MatroskaSpec::CbSubsamplingHorz(_) => "Cb Subsampling Horz",
        MatroskaSpec::CbSubsamplingVert(_) => "Cb Subsampling Vert",
        MatroskaSpec::ChromaSitingHorz(_) => "Chroma Siting Horz",
        MatroskaSpec::ChromaSitingVert(_) => "Chroma Siting Vert",
        MatroskaSpec::ChromaSubsamplingHorz(_) => "Chroma Subsampling Horz",
        MatroskaSpec::ChromaSubsamplingVert(_) => "Chroma Subsampling Vert",
        MatroskaSpec::MasteringMetadata(Master::Start) => "Mastering Metadata Start",
        MatroskaSpec::MasteringMetadata(Master::End) => "Mastering Metadata End",
        MatroskaSpec::MasteringMetadata(Master::Full(_)) => "Mastering Metadata Full",
        MatroskaSpec::LuminanceMax(_) => "Luminance Max",
        MatroskaSpec::LuminanceMin(_) => "Luminance Min",
        MatroskaSpec::PrimaryBChromaticityX(_) => "Primary B Chromaticity X",
        MatroskaSpec::PrimaryBChromaticityY(_) => "Primary B Chromaticity Y",
        MatroskaSpec::PrimaryGChromaticityX(_) => "Primary G Chromaticity X",
        MatroskaSpec::PrimaryGChromaticityY(_) => "Primary G Chromaticity Y",
        MatroskaSpec::PrimaryRChromaticityX(_) => "Primary R Chromaticity X",
        MatroskaSpec::PrimaryRChromaticityY(_) => "Primary R Chromaticity Y",
        MatroskaSpec::WhitePointChromaticityX(_) => "White Point Chromaticity X",
        MatroskaSpec::WhitePointChromaticityY(_) => "White Point Chromaticity Y",
        MatroskaSpec::MatrixCoefficients(_) => "Matrix Coefficients",
        MatroskaSpec::MaxCLL(_) => "Max CLL",
        MatroskaSpec::MaxFALL(_) => "Max FALL",
        MatroskaSpec::Primaries(_) => "Primaries",
        MatroskaSpec::Range(_) => "Range",
        MatroskaSpec::TransferCharacteristics(_) => "Transfer Characteristics",
        MatroskaSpec::DisplayHeight(_) => "Display Height",
        MatroskaSpec::DisplayUnit(_) => "Display Unit",
        MatroskaSpec::DisplayWidth(_) => "Display Width",
        MatroskaSpec::FieldOrder(_) => "Field Order",
        MatroskaSpec::FlagInterlaced(_) => "Flag Interlaced",
        MatroskaSpec::FrameRate(_) => "Frame Rate",
        MatroskaSpec::GammaValue(_) => "Gamma Value",
        MatroskaSpec::OldStereoMode(_) => "Old Stereo Mode",
        MatroskaSpec::PixelCropBottom(_) => "Pixel Crop Bottom",
        MatroskaSpec::PixelCropLeft(_) => "Pixel Crop Left",
        MatroskaSpec::PixelCropRight(_) => "Pixel Crop Right",
        MatroskaSpec::PixelCropTop(_) => "Pixel Crop Top",
        MatroskaSpec::PixelHeight(_) => "Pixel Height",
        MatroskaSpec::PixelWidth(_) => "Pixel Width",
        MatroskaSpec::Projection(Master::Start) => "Projection Start",
        MatroskaSpec::Projection(Master::End) => "Projection End",
        MatroskaSpec::Projection(Master::Full(_)) => "Projection Full",
        MatroskaSpec::ProjectionPosePitch(_) => "Projection Pose Pitch",
        MatroskaSpec::ProjectionPoseRoll(_) => "Projection Pose Roll",
        MatroskaSpec::ProjectionPoseYaw(_) => "Projection Pose Yaw",
        MatroskaSpec::ProjectionPrivate(_) => "Projection Private",
        MatroskaSpec::ProjectionType(_) => "Projection Type",
        MatroskaSpec::StereoMode(_) => "Stereo Mode",
        MatroskaSpec::UncompressedFourCC(_) => "Uncompressed FourCC",
        MatroskaSpec::Crc32(_) => "Crc32",
        MatroskaSpec::Void(_) => "Void",
        MatroskaSpec::RawTag(_, _) => "Raw Tag",
    }
}
