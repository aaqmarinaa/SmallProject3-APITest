import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('Employee/Dependants/POST Save Dependants'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'success', 'Successfully Saved')

WS.verifyElementPropertyValue(response, 'sequenceNumber', 2)

response = WS.sendRequest(findTestObject('Employee/Dependants/GET Employee Dependants'))

WS.verifyElementsCount(response, 'data', 2)

WS.verifyElementPropertyValue(response, 'data[1].name', 'coba 2')

WS.verifyElementPropertyValue(response, 'data[1].relationship', 'Daughter')

WS.verifyElementPropertyValue(response, 'data[1].dob', '2002-05-18')

WS.verifyElementPropertyValue(response, 'data[1].sequenceNumber', '2')

