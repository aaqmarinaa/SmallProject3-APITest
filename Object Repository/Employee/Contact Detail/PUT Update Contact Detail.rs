<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT Update Contact Detail</name>
   <tag></tag>
   <elementGuidId>8946f2fd-fd1f-4ab2-a9f4-49095a340ae8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;id\&quot;:\&quot;268\&quot;,\n    \&quot;addressStreet1\&quot;:\&quot;Jl. Palagan Tentara Pelajaran\&quot;,\n    \&quot;city\&quot;:\&quot;Yogyakarta\&quot;,\n    \&quot;state\&quot;:\&quot;D.I. Yogyakarta\&quot;,\n    \&quot;zip\&quot;:\&quot;18787\&quot;,\n    \&quot;country\&quot;:\&quot;Indonesia\&quot;,\n    \&quot;homeTelephone\&quot;:\&quot;021-12345678\&quot;,\n    \&quot;mobile\&quot;:\&quot;087853269999\&quot;,\n  \t\&quot;workTelephone\&quot;:\&quot;021-374792384\&quot;,\n    \&quot;workEmail\&quot;:\&quot;almas4@cilsy.id\&quot;,\n    \&quot;otherEmail\&quot;:\&quot;almas4@gmail.com\&quot;\n}&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.token}</value>
   </httpHeaderProperties>
   <katalonVersion>7.7.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/api/v1/employee/268/contact-detail</restUrl>
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
