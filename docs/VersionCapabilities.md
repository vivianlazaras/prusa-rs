# VersionCapabilities

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**upload_by_put** | Option<**bool**> | The printer supports uploading GCodes by the PUT method (as described in this document). It is capable of doing the PUT and HEAD to /api/v1/files/{storage}/{path} and it is capable of answering the /api/v1/storage endpoint.  In absence of this capability, client MAY opt to try the legacy \"octoprint\" POST to /api/files/{storage}.  | [optional][default to false]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


