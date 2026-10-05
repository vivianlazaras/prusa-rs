# Job

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**serial_print** | Option<**bool**> | Whether the printer is printing from the serial line | [optional][default to false]
**file** | Option<[**models::JobFilePrintFile**](JobFilePrintFile.md)> |  | [optional]
**id** | **i32** |  | 
**state** | **State** |  (enum: PRINTING, PAUSED, FINISHED, STOPPED, ERROR) | 
**progress** | **f64** | Percents | 
**time_remaining** | Option<**i32**> | Seconds | [optional]
**time_printing** | **i32** | Seconds | 
**inaccurate_estimates** | Option<**bool**> | Whether the time estimates are accurate or inaccurate | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


