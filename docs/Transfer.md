# Transfer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | **Type** |  (enum: NO_TRANSFER, FROM_WEB, FROM_CONNECT, FROM_PRINTER, FROM_SLICER, FROM_CLIENT, TO_CONNECT, TO_CLIENT) | 
**display_name** | **String** | Long Filename | 
**path** | **String** |  | 
**url** | Option<**String**> |  | [optional]
**size** | Option<**String**> | Bytes | [optional]
**progress** | **f64** | Percents | 
**transferred** | **i32** | Transferred data in bytes | 
**time_remaining** | Option<**i32**> | Seconds | [optional]
**time_transferring** | **i32** | Seconds | 
**to_print** | **bool** | Whether or not print after finishing transfer (upload) | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


