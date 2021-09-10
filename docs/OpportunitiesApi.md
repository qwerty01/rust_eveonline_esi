# \OpportunitiesApi

All URIs are relative to *https://esi.evetech.net/latest*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_characters_character_id_opportunities**](OpportunitiesApi.md#get_characters_character_id_opportunities) | **GET** /characters/{character_id}/opportunities/ | Get a character's completed tasks
[**get_opportunities_groups**](OpportunitiesApi.md#get_opportunities_groups) | **GET** /opportunities/groups/ | Get opportunities groups
[**get_opportunities_groups_group_id**](OpportunitiesApi.md#get_opportunities_groups_group_id) | **GET** /opportunities/groups/{group_id}/ | Get opportunities group
[**get_opportunities_tasks**](OpportunitiesApi.md#get_opportunities_tasks) | **GET** /opportunities/tasks/ | Get opportunities tasks
[**get_opportunities_tasks_task_id**](OpportunitiesApi.md#get_opportunities_tasks_task_id) | **GET** /opportunities/tasks/{task_id}/ | Get opportunities task



## get_characters_character_id_opportunities

> Vec<crate::models::GetCharactersCharacterIdOpportunities200Ok> get_characters_character_id_opportunities(character_id, datasource, if_none_match, token)
Get a character's completed tasks

Return a list of tasks finished by a character  --- Alternate route: `/dev/characters/{character_id}/opportunities/`  Alternate route: `/legacy/characters/{character_id}/opportunities/`  Alternate route: `/v1/characters/{character_id}/opportunities/`  --- This route is cached for up to 3600 seconds

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**character_id** | **i32** | An EVE character ID | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**token** | Option<**String**> | Access token to use if unable to set a header |  |

### Return type

[**Vec<crate::models::GetCharactersCharacterIdOpportunities200Ok>**](get_characters_character_id_opportunities_200_ok.md)

### Authorization

[evesso](../README.md#evesso)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_opportunities_groups

> Vec<i32> get_opportunities_groups(datasource, if_none_match)
Get opportunities groups

Return a list of opportunities groups  --- Alternate route: `/dev/opportunities/groups/`  Alternate route: `/legacy/opportunities/groups/`  Alternate route: `/v1/opportunities/groups/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_opportunities_groups_group_id

> crate::models::GetOpportunitiesGroupsGroupIdOk get_opportunities_groups_group_id(group_id, accept_language, datasource, if_none_match, language)
Get opportunities group

Return information of an opportunities group  --- Alternate route: `/dev/opportunities/groups/{group_id}/`  Alternate route: `/legacy/opportunities/groups/{group_id}/`  Alternate route: `/v1/opportunities/groups/{group_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **i32** | ID of an opportunities group | [required] |
**accept_language** | Option<**String**> | Language to use in the response |  |[default to en]
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |
**language** | Option<**String**> | Language to use in the response, takes precedence over Accept-Language |  |[default to en]

### Return type

[**crate::models::GetOpportunitiesGroupsGroupIdOk**](get_opportunities_groups_group_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_opportunities_tasks

> Vec<i32> get_opportunities_tasks(datasource, if_none_match)
Get opportunities tasks

Return a list of opportunities tasks  --- Alternate route: `/dev/opportunities/tasks/`  Alternate route: `/legacy/opportunities/tasks/`  Alternate route: `/v1/opportunities/tasks/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

**Vec<i32>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_opportunities_tasks_task_id

> crate::models::GetOpportunitiesTasksTaskIdOk get_opportunities_tasks_task_id(task_id, datasource, if_none_match)
Get opportunities task

Return information of an opportunities task  --- Alternate route: `/dev/opportunities/tasks/{task_id}/`  Alternate route: `/legacy/opportunities/tasks/{task_id}/`  Alternate route: `/v1/opportunities/tasks/{task_id}/`  --- This route expires daily at 11:05

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **i32** | ID of an opportunities task | [required] |
**datasource** | Option<**String**> | The server name you would like data from |  |[default to tranquility]
**if_none_match** | Option<**String**> | ETag from a previous request. A 304 will be returned if this matches the current ETag |  |

### Return type

[**crate::models::GetOpportunitiesTasksTaskIdOk**](get_opportunities_tasks_task_id_ok.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

