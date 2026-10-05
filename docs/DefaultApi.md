# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**capture_camera_snapshot**](DefaultApi.md#capture_camera_snapshot) | **POST** /api/v1/cameras/{id}/snap | Make a snapshot with the camera
[**configure_camera**](DefaultApi.md#configure_camera) | **POST** /api/v1/cameras/{id} | Setup a new camera or fix a broken one
[**continue_job**](DefaultApi.md#continue_job) | **PUT** /api/v1/job/{id}/continue | continue job
[**delete_camera**](DefaultApi.md#delete_camera) | **DELETE** /api/v1/cameras/{id} | Delete a camera
[**delete_file**](DefaultApi.md#delete_file) | **DELETE** /api/v1/files/{storage}/{path} | Delete a file or folder
[**get_api_version**](DefaultApi.md#get_api_version) | **GET** /api/version | api version information
[**get_camera**](DefaultApi.md#get_camera) | **GET** /api/v1/cameras/{id} | Get current settings and properties of specific camera
[**get_camera_snapshot**](DefaultApi.md#get_camera_snapshot) | **GET** /api/v1/cameras/{id}/snap | Return a captured image from the camera with a given id
[**get_default_camera_snapshot**](DefaultApi.md#get_default_camera_snapshot) | **GET** /api/v1/cameras/snap | Return a captured image from the default camera
[**get_file_metadata**](DefaultApi.md#get_file_metadata) | **GET** /api/v1/files/{storage}/{path} | File or folder metadata
[**get_job**](DefaultApi.md#get_job) | **GET** /api/v1/job | job info
[**get_printer_info**](DefaultApi.md#get_printer_info) | **GET** /api/v1/info | printer information
[**get_printer_status**](DefaultApi.md#get_printer_status) | **GET** /api/v1/status | printer, job and transfer telemetry info
[**get_storage**](DefaultApi.md#get_storage) | **GET** /api/v1/storage | storage info
[**get_transfer**](DefaultApi.md#get_transfer) | **GET** /api/v1/transfer | transfer info
[**get_update_info**](DefaultApi.md#get_update_info) | **GET** /api/v1/update/{env} | Retrieve information about available update of given environment
[**head_file**](DefaultApi.md#head_file) | **HEAD** /api/v1/files/{storage}/{path} | file presence and state check
[**list_cameras**](DefaultApi.md#list_cameras) | **GET** /api/v1/cameras | Get a list of active cameras and its properties
[**pause_job**](DefaultApi.md#pause_job) | **PUT** /api/v1/job/{id}/pause | pause job
[**register_camera**](DefaultApi.md#register_camera) | **POST** /api/v1/cameras/{id}/connection | Register a camera to Connect
[**reset_camera_config**](DefaultApi.md#reset_camera_config) | **DELETE** /api/v1/cameras/{id}/config | Reset settings of a camera
[**resume_job**](DefaultApi.md#resume_job) | **PUT** /api/v1/job/{id}/resume | resume job
[**set_camera_order**](DefaultApi.md#set_camera_order) | **PUT** /api/v1/cameras | List of cameras in intended order
[**start_print**](DefaultApi.md#start_print) | **POST** /api/v1/files/{storage}/{path} | Start print of file if there's no print job running
[**stop_job**](DefaultApi.md#stop_job) | **DELETE** /api/v1/job/{id} | stop job
[**stop_transfer**](DefaultApi.md#stop_transfer) | **DELETE** /api/v1/transfer/{id} | stop transfer
[**unregister_camera**](DefaultApi.md#unregister_camera) | **DELETE** /api/v1/cameras/{id}/connection | Un-register a camera from Connect
[**update_camera_config**](DefaultApi.md#update_camera_config) | **PATCH** /api/v1/cameras/{id}/config | Set new settings to a working camera
[**update_environment**](DefaultApi.md#update_environment) | **POST** /api/v1/update/{env} | Update given environment
[**upload_file**](DefaultApi.md#upload_file) | **PUT** /api/v1/files/{storage}/{path} | upload file or create folder



## capture_camera_snapshot

> std::path::PathBuf capture_camera_snapshot(id)
Make a snapshot with the camera

Can be manually done only during camera initialization or in manual mode

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## configure_camera

> configure_camera(id, camera_config_set)
Setup a new camera or fix a broken one

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |
**camera_config_set** | [**CameraConfigSet**](CameraConfigSet.md) | Camera configuration to set. | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## continue_job

> continue_job(id)
continue job

Continue in job with given id after timelapse capture

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | job id | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_camera

> delete_camera(id)
Delete a camera

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_file

> delete_file(storage, path, accept_language, accept, force)
Delete a file or folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage** | **String** | The target storage | [required] |
**path** | **String** | Path to the file | [required] |
**accept_language** | Option<**String**> | Defines a language of the response |  |
**accept** | Option<**String**> | Preferred content-type of response - application/json or text/html, all other are returned as text/plain |  |[default to text/plain]
**force** | Option<**String**> | Whether to force delete non-empty folder |  |[default to ?0]

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_api_version

> models::Version get_api_version()
api version information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Version**](Version.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_camera

> models::CameraConfig get_camera(id)
Get current settings and properties of specific camera

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

[**models::CameraConfig**](CameraConfig.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_camera_snapshot

> std::path::PathBuf get_camera_snapshot(id)
Return a captured image from the camera with a given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_default_camera_snapshot

> std::path::PathBuf get_default_camera_snapshot()
Return a captured image from the default camera

### Parameters

This endpoint does not need any parameter.

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: image/png, text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_metadata

> models::GetFileMetadata200Response get_file_metadata(storage, path, accept_language, accept)
File or folder metadata

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage** | **String** | The target storage | [required] |
**path** | **String** | Path to the file | [required] |
**accept_language** | Option<**String**> | Defines a language of the response |  |
**accept** | Option<**String**> | Preferred content-type of response - application/json or text/html, all other are returned as text/plain |  |[default to text/plain]

### Return type

[**models::GetFileMetadata200Response**](getFileMetadata_200_response.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_job

> models::Job get_job()
job info

Returns info about current job

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Job**](Job.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_printer_info

> models::Info get_printer_info()
printer information

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Info**](Info.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_printer_status

> models::GetPrinterStatus200Response get_printer_status()
printer, job and transfer telemetry info

All values except printer are optional

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::GetPrinterStatus200Response**](getPrinterStatus_200_response.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage

> models::GetStorage200Response get_storage(accept_language)
storage info

Returns info about each available file storage (e.g. SD Card or local storage)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_language** | Option<**String**> | Defines a language of the response |  |

### Return type

[**models::GetStorage200Response**](getStorage_200_response.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transfer

> models::Transfer get_transfer()
transfer info

Returns info about current transfer

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Transfer**](Transfer.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_update_info

> models::PrusaLinkPackage get_update_info(env)
Retrieve information about available update of given environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env** | **String** | The target environment (prusalink or system) for update | [required] |

### Return type

[**models::PrusaLinkPackage**](PrusaLinkPackage.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## head_file

> head_file(storage, path, accept_language, accept)
file presence and state check

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage** | **String** | The target storage | [required] |
**path** | **String** | Path to the file | [required] |
**accept_language** | Option<**String**> | Defines a language of the response |  |
**accept** | Option<**String**> | Preferred content-type of response - application/json or text/html, all other are returned as text/plain |  |[default to text/plain]

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_cameras

> Vec<models::Camera> list_cameras()
Get a list of active cameras and its properties

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Camera>**](Camera.md)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pause_job

> pause_job(id)
pause job

Pause job with given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | job id | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## register_camera

> register_camera(id)
Register a camera to Connect

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_camera_config

> reset_camera_config(id)
Reset settings of a camera

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## resume_job

> resume_job(id)
resume job

Resume job with given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | job id | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_camera_order

> set_camera_order(request_body)
List of cameras in intended order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_body** | [**Vec<String>**](String.md) | Printer/camera IDs in the intended display order. | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## start_print

> start_print(storage, path, accept_language, accept)
Start print of file if there's no print job running

Body is ignored

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage** | **String** | The target storage | [required] |
**path** | **String** | Path to the file | [required] |
**accept_language** | Option<**String**> | Defines a language of the response |  |
**accept** | Option<**String**> | Preferred content-type of response - application/json or text/html, all other are returned as text/plain |  |[default to text/plain]

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_job

> stop_job(id)
stop job

Stop job with given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | job id | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## stop_transfer

> stop_transfer(id)
stop transfer

Stop transfer with given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | transfer id | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unregister_camera

> unregister_camera(id)
Un-register a camera from Connect

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_camera_config

> update_camera_config(id, camera_config_set)
Set new settings to a working camera

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the camera | [required] |
**camera_config_set** | [**CameraConfigSet**](CameraConfigSet.md) | Camera configuration to set. | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_environment

> update_environment(env)
Update given environment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**env** | **String** | The target environment (prusalink or system) for update | [required] |

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file

> upload_file(storage, path, body, accept_language, accept, content_length, content_type, print_after_upload, overwrite)
upload file or create folder

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**storage** | **String** | The target storage | [required] |
**path** | **String** | Path to the file | [required] |
**body** | **std::path::PathBuf** | File contents to upload. | [required] |
**accept_language** | Option<**String**> | Defines a language of the response |  |
**accept** | Option<**String**> | Preferred content-type of response - application/json or text/html, all other are returned as text/plain |  |[default to text/plain]
**content_length** | Option<**i32**> | Length of file to upload |  |
**content_type** | Option<**String**> | Type of uploaded media |  |[default to application/octet-stream]
**print_after_upload** | Option<**String**> | Whether to start printing the file after upload |  |[default to ?0]
**overwrite** | Option<**String**> | Whether to overwrite already existing files |  |[default to ?0]

### Return type

 (empty response body)

### Authorization

[digestAuth](../README.md#digestAuth)

### HTTP request headers

- **Content-Type**: application/octet-stream
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

