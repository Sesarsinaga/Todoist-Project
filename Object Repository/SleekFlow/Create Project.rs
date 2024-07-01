<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Project</name>
   <tag></tag>
   <elementGuidId>10ea4bcc-e71e-46d6-84d9-7b70fa2ddde1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;with_web_static_version\&quot;: true,\n    \&quot;limit_notes\&quot;: 1,\n    \&quot;max_notes\&quot;: 5,\n    \&quot;with_dateist_version\&quot;: 1,\n    \&quot;resource_types\&quot;: [\n        \&quot;all\&quot;\n    ],\n    \&quot;sync_token\&quot;: \&quot;RW1RhfD6Fvvc_D3nZkBHnndWpux9YORB_BoSAW9Cu7x_KHm8DE7MVccviUyOCUxjR9lG7gZCHjul2NRpqvHpBCb1ey00o3tmA2qhvKAdE1LnoiE\&quot;,\n    \&quot;commands\&quot;: [\n        {\n            \&quot;type\&quot;: \&quot;project_add\&quot;,\n            \&quot;uuid\&quot;: \&quot;${uuid}\&quot;,\n            \&quot;temp_id\&quot;: \&quot;${tempId}\&quot;,\n            \&quot;args\&quot;: {\n                \&quot;name\&quot;: \&quot;${projectName}\&quot;,\n                \&quot;color\&quot;: \&quot;charcoal\&quot;,\n                \&quot;parent_id\&quot;: null,\n                \&quot;child_order\&quot;: 4,\n                \&quot;is_favorite\&quot;: false,\n                \&quot;view_style\&quot;: \&quot;list\&quot;,\n                \&quot;is_invite_only\&quot;: false,\n                \&quot;can_assign_tasks\&quot;: false\n            }\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accept</name>
      <type>Main</type>
      <value>*/*</value>
      <webElementGuid>5d2a8b83-6ee7-40ce-abce-9cd6e6c2ea5d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accept-language</name>
      <type>Main</type>
      <value>en-US,en;q=0.9</value>
      <webElementGuid>9846d39c-2c25-406e-bb73-7d21d1703a75</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>c3833682-cec5-4dd2-828a-d717cba94b78</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>content-type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>10bf62ff-7942-4bee-8daa-6ebde03da95b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>cookie</name>
      <type>Main</type>
      <value>_ga=GA1.1.359572062.1719412571; ps_mode=trackingV1; _clck=19jnq02%7C2%7Cfmy%7C0%7C1638; _ga_3WJ7YJ0FM2=GS1.1.1719412571.1.1.1719413361.0.0.0; _ga_47KPF4T19V=GS1.1.1719412571.1.1.1719413361.0.0.0; _clsk=n9o6c9%7C1719413361855%7C2%7C1%7Cu.clarity.ms%2Fcollect; csrf=40309dbf9ea843379dc8f5dc02a11ed5; _ga=GA1.3.359572062.1719412571; _gid=GA1.3.68493969.1719413389; ki_r=; _rdt_uuid=1719412571504.452d02cc-3e07-4168-93e0-b886bd128e3f; tduser=v4.public.eyJ1c2VyX2lkIjogNDk2OTYwMzMsICJleHAiOiAiMjAyNC0wNy0xMFQxNjoxMDowNCswMDowMCJ9yAIt1kZ1HgijZJ40M4hKwuWnXHBBYo5crSkZ-rOsPDjl20q2SfWpLqpwn0HeF6-YXk-zT-GmijoSAzi7LqlCAQ; todoistd=&quot;l4AtCZ+jOJHJ0m42ylXIHQQTvo0=?pCHK=gASVJAAAAAAAAACMIDM1YWNmNjcxNGZiZWQxMzk0MTcwODJiMWY0M2RmMWIylC4=&amp;user_id=gASVBgAAAAAAAABKIU32Ai4=&quot;; ki_t=1719413392988%3B1719413392988%3B1719418214325%3B1%3B7; _dd_s=logs=1&amp;id=6b2c66c9-d1cf-43df-9943-28ac8a1fe4bd&amp;created=1719413383224&amp;expire=1719419834338; _ga_M6V9BEQD2J=GS1.1.1719413362.1.1.1719418934.0.0.0</value>
      <webElementGuid>60dfc6d7-c9e1-4cb6-be51-a9add1f2aed7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>doist-locale</name>
      <type>Main</type>
      <value>en</value>
      <webElementGuid>6b220907-a516-411f-8b74-c9864d8acda5</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>doist-os</name>
      <type>Main</type>
      <value>Windows</value>
      <webElementGuid>22e7b638-8eca-4a5e-bf31-5b320fe82396</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>doist-platform</name>
      <type>Main</type>
      <value>web</value>
      <webElementGuid>0f5b2f8e-5b6a-47bf-b9c6-634da280352d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>doist-screen</name>
      <type>Main</type>
      <value>1280x672</value>
      <webElementGuid>3a81cf84-5459-48c3-a40c-585a27006f54</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>doist-version</name>
      <type>Main</type>
      <value>6255</value>
      <webElementGuid>8cc8e56d-a1c4-45fd-a905-54e9901c16af</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>origin</name>
      <type>Main</type>
      <value>https://app.todoist.com</value>
      <webElementGuid>f03c8329-7cf2-4cd6-8585-19769a02e418</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>priority</name>
      <type>Main</type>
      <value>u=1, i</value>
      <webElementGuid>d7cf31ab-018d-4336-b8fc-bf7ffb0b4c82</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>referer</name>
      <type>Main</type>
      <value>https://app.todoist.com/app/project/simulate-_1719418206761</value>
      <webElementGuid>5966ba5c-6904-46c3-bb9f-d9a81a4f05ef</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>sec-ch-ua</name>
      <type>Main</type>
      <value>&quot;Not/A)Brand&quot;;v=&quot;8&quot;, &quot;Chromium&quot;;v=&quot;126&quot;, &quot;Google Chrome&quot;;v=&quot;126&quot;</value>
      <webElementGuid>4c9b441b-97be-42d9-a8fe-149397b4ac71</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>sec-ch-ua-mobile</name>
      <type>Main</type>
      <value>?0</value>
      <webElementGuid>dc00fdc5-fe1a-486e-a344-bbeea289856d</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>sec-ch-ua-platform</name>
      <type>Main</type>
      <value>&quot;Windows&quot;</value>
      <webElementGuid>6eeed88d-e298-4c96-95e6-8cf628e8914a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>sec-fetch-dest</name>
      <type>Main</type>
      <value>empty</value>
      <webElementGuid>5c65598e-614d-4ac9-a1a7-235a1c6e944e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>sec-fetch-mode</name>
      <type>Main</type>
      <value>cors</value>
      <webElementGuid>ea922605-bda0-4106-8e99-a9f691ff11fc</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>sec-fetch-site</name>
      <type>Main</type>
      <value>same-origin</value>
      <webElementGuid>4466da0a-ed5e-4121-b190-b655863ad2d7</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>todoist-partial-sync</name>
      <type>Main</type>
      <value>true</value>
      <webElementGuid>cadeb823-3d68-4ebb-bc3d-f7a20e6cf541</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>user-agent</name>
      <type>Main</type>
      <value>Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36</value>
      <webElementGuid>0440d669-e45f-44fc-806d-b68bf7bcd6e3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-csrftoken</name>
      <type>Main</type>
      <value>1719418206768</value>
      <webElementGuid>120a6041-ca27-4715-a435-4119aa8e606e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-todoist-client-id</name>
      <type>Main</type>
      <value>7b10c708-9b7c-6288-5d7d-19112c1340e2</value>
      <webElementGuid>51c0e409-14f3-422e-9163-0fe431803e36</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-todoist-webapp-version</name>
      <type>Main</type>
      <value>6255</value>
      <webElementGuid>c1080dde-0224-45f0-9711-375e50e8d72c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://app.todoist.com/api/v9.1/sync</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
