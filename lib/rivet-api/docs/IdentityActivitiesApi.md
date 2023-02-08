# \IdentityActivitiesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**identity_activities_list**](IdentityActivitiesApi.md#identity_activities_list) | **GET** /activities | 



## identity_activities_list

> crate::models::IdentityListActivitiesOutput identity_activities_list(watch_index)


Returns an overview of all players currently online or in game.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**watch_index** | Option<**String**> |  |  |

### Return type

[**crate::models::IdentityListActivitiesOutput**](IdentityListActivitiesOutput.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
