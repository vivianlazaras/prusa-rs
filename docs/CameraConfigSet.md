# CameraConfigSet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | Name of the camera | [optional]
**trigger_scheme** | Option<**TriggerScheme**> | When the snapshot is taken (enum: TEN_SEC, THIRTY_SEC, SIXTY_SEC, EACH_LAYER, FIFTH_LAYER, MANUAL) | [optional]
**resolution** | Option<[**models::CameraConfigSetResolution**](CameraConfigSetResolution.md)> |  | [optional]
**rotation** | Option<**i32**> | Current rotation of the output image | [optional]
**focus** | Option<**f64**> | Focus of the camera (0.0 - 1.0) | [optional]
**exposure** | Option<**f64**> |  | [optional]
**send_to_connect** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


