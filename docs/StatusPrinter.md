# StatusPrinter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**state** | **State** |  (enum: IDLE, BUSY, PRINTING, PAUSED, FINISHED, STOPPED, ERROR, ATTENTION, READY) | 
**temp_nozzle** | Option<**f64**> |  | [optional]
**target_nozzle** | Option<**f64**> |  | [optional]
**temp_bed** | Option<**f64**> |  | [optional]
**target_bed** | Option<**f64**> |  | [optional]
**axis_x** | Option<**f64**> | Available only when printer is not moving | [optional]
**axis_y** | Option<**f64**> | Available only when printer is not moving | [optional]
**axis_z** | Option<**f64**> |  | [optional]
**flow** | Option<**i32**> |  | [optional]
**speed** | Option<**i32**> |  | [optional]
**fan_hotend** | Option<**i32**> |  | [optional]
**fan_print** | Option<**i32**> |  | [optional]
**status_printer** | Option<[**models::StatusPrinterStatusPrinter**](StatusPrinterStatusPrinter.md)> |  | [optional]
**status_connect** | Option<[**models::StatusPrinterStatusPrinter**](StatusPrinterStatusPrinter.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


