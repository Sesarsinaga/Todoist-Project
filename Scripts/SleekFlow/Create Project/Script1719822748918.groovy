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
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

import groovy.json.JsonSlurper
import java.util.ArrayList

//Part 1
'=================================== Login ====================================='

Login = findTestObject('Object Repository/SleekFlow/Login')

'Request Body Auth'
request_body = (Login.getHttpBody() as String)
WS.comment('request body ' + request_body)


'Response Body Auth'
response_body = WS.sendRequest(Login)
WS.comment('response body ' + response_body.getResponseText())

'Validate Status Code'
 WS.verifyResponseStatusCode(response_body, 200)

'Get Parsing Data'
JsonSlurper slurper = new JsonSlurper()
Map parsedJson = slurper.parseText(response_body.getResponseText())

GlobalVariable.G_token = parsedJson.token

println ("... Global Variable now is : "+ GlobalVariable.G_token)

'=================================== GET Random Alphanumeric ====================================='

def generateRandomAlphaNumeric(int length) {
	String characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	Random random = new Random();
	StringBuilder sb = new StringBuilder(length);

	for (int i = 0; i < length; i++) {
		int randomIndex = random.nextInt(characters.length());
		sb.append(characters.charAt(randomIndex));
	}

	return sb.toString();
}

def randomAlphaNumeric = generateRandomAlphaNumeric(20)

GlobalVariable.G_dynamicString = randomAlphaNumeric

'=================================== Create Project ====================================='

createProject = findTestObject('Object Repository/SleekFlow/Create Project', [
	"token" : GlobalVariable.G_token,
	"uuid" : GlobalVariable.G_dynamicString,
	"tempId" : GlobalVariable.G_dynamicString
	])

'Request Body Auth'
request_body = (createProject.getHttpBody() as String)
WS.comment('request body ' + request_body)


'Response Body Auth'
response_body = WS.sendRequest(createProject)
WS.comment('response body ' + response_body.getResponseText())

'Validate Status Code'
 WS.verifyResponseStatusCode(response_body, 200)
 
 Mobile.delay(5)
 
 'Get Parsing Data'
 JsonSlurper slurper = new JsonSlurper()
 Map parsedJson = slurper.parseText(response_body.getResponseText())
 
 
 
 
 
 
 
 //Part2
 
 

 
 
 
 
 
 
 
 
 