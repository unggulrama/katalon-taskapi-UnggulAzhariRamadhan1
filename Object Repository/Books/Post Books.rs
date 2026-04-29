<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Books</name>
   <tag></tag>
   <elementGuidId>c7a7fa0d-dab7-45e4-98e9-16ae14c6a325</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;:1,\n  \&quot;title\&quot;:\&quot;${titleInput}\&quot;,\n  \&quot;description\&quot;:\&quot;Lorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\\n\&quot;,\n  \&quot;pageCount\&quot;:100,\n  \&quot;excerpt\&quot;:\&quot;Lorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\\n\&quot;,\n  \&quot;publishDate\&quot;:\&quot;2026-04-28T12:07:18.5407338+00:00\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>41b7ffed-0e80-462b-bdd0-85ea30abee3d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.4.3</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/v1/Books</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Book 1'</defaultValue>
      <description></description>
      <id>1a5fb3c0-d6bf-4af6-ae5b-e58081a8d521</id>
      <masked>false</masked>
      <name>titleInput</name>
   </variables>
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
