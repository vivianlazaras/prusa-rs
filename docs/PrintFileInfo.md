# PrintFileInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Short Filename | 
**read_only** | **bool** |  | 
**size** | Option<**i32**> | Available for files only, not for folders | [optional]
**r#type** | **Type** | File could be print file, firmware file, other (e.g. configuration) file, or folder (enum: PRINT_FILE, FIRMWARE, FILE, FOLDER) | 
**m_timestamp** | **i32** | Timestamp in seconds | 
**display_name** | Option<**String**> | Long Filename | [optional]
**refs** | Option<[**models::PrintFileRefs**](PrintFileRefs.md)> |  | [optional]
**meta** | Option<[**models::PrintFileMetadata**](PrintFileMetadata.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


