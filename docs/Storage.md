# Storage

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the storage, based on selected language | [optional]
**r#type** | **Type** | Storage source (enum: LOCAL, SDCARD, USB) | 
**path** | **String** | Path to storage (not display path) | 
**print_files** | Option<**i32**> | Size of all print files in bytes | [optional]
**system_files** | Option<**i32**> | Size of all system files in bytes | [optional]
**free_space** | Option<**i32**> | System free space in bytes | [optional]
**total_space** | Option<**i32**> | System total space in bytes | [optional]
**available** | **bool** | Whether the storage is available or not | 
**read_only** | Option<**bool**> | Whether the storage is read only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


