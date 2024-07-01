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

//Part 1 : Create Project Using API
'=================================== Login ====================================='

Mobile.callTestCase(findTestCase('Test Cases/SleekFlow/Login'), null, FailureHandling.STOP_ON_FAILURE)

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

System.out.println("Global Variable ": GlobalVariable.G_dynamicString)

'=================================== Create Project ====================================='

createProject = findTestObject('Object Repository/SleekFlow/Create Project', [
	"token" : GlobalVariable.G_token,
	"uuid" : GlobalVariable.G_dynamicString,
	"tempId" : GlobalVariable.G_dynamicString,
	"projectName" : GlobalVariable.G_projectName
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
 // --
 
 //Part2 : Login into Mobile Application
 
 'Choose Continue with email'
 Mobile.tap(findTestObject('Object Repository/SleekFlow/Button - Continue with Email'), 0, FailureHandling.STOP_ON_FAILURE)
 
 'Click Login with Email'
 Mobile.tap(findTestObject('Object Repository/SleekFlow/TextView - Login with Email'), 0, FailureHandling.STOP_ON_FAILURE)
 
 'Input Email'
 Mobile.setText(findTestObject('Object Repository/SleekFlow/EditText - Input Email'), GlobalVariable.G_email, 0, FailureHandling.STOP_ON_FAILURE)
 
 'Input Password'
 Mobile.setText(findTestObject('Object Repository/SleekFlow/EditText - Input Password'), GlobalVariable.G_psswrd, 0, FailureHandling.STOP_ON_FAILURE)
 
 'Click Log in'
 Mobile.tap(findTestObject('Object Repository/SleekFlow/TextView - Log in'), 0, FailureHandling.STOP_ON_FAILURE)
 
 'Wait for element present'
 Mobile.waitForElementPresent(findTestObject('Object Repository/SleekFlow/TextView - Browse'), 0, FailureHandling.STOP_ON_FAILURE)
 
 'Click Tab Browse'
 Mobile.tap(findTestObject('Object Repository/SleekFlow/TextView - Browse'), 0, FailureHandling.STOP_ON_FAILURE)
 
 'Get Project Name on Device'
 def projectNameDevice = Mobile.getText(findTestObject('Object Repository/SleekFlow/TextView - Project Name on Device'), 0, FailureHandling.STOP_ON_FAILURE)
 
'Verify Project Name on Device and on API'
Mobile.verifyMatch(projectNameDevice, GlobalVariable.G_projectName, false, FailureHandling.STOP_ON_FAILURE)
// --

//Part 3 : 
'Click Created Project'
Mobile.tap(findTestObject('Object Repository/SleekFlow/TextView - Created Project'), 0, FailureHandling.STOP_ON_FAILURE)

'Tap Create New Task'
Mobile.tap(findTestObject('Object Repository/SleekFlow/ImageButton - Create Task'), 0, FailureHandling.STOP_ON_FAILURE)

'Input Title'
Mobile.setText(findTestObject('Object Repository/SleekFlow/EditText - Input Title'), "Create New Task", 0, FailureHandling.STOP_ON_FAILURE)

'Input Description'
Mobile.setText(findTestObject('Object Repository/SleekFlow/EditText - Input Description'), "Test on SleekFlow", 0, FailureHandling.STOP_ON_FAILURE)

'Tap Due Date'
Mobile.tap(findTestObject('Object Repository/SleekFlow/TextView - Due date'), 0, FailureHandling.STOP_ON_FAILURE)

'Choose Choice of Due Date'
Mobile.tap(findTestObject('Object Repository/SleekFlow/TextView - Choose Due Date for Next week'), 0, FailureHandling.STOP_ON_FAILURE)

'Tap Next For Create New Task'
Mobile.tap(findTestObject('Object Repository/SleekFlow/ImageView - Next For Create Task'), 0, FailureHandling.STOP_ON_FAILURE)

'Press Back'
Mobile.pressBack()

def taskName = Mobile.getText(findTestObject('Object Repository/SleekFlow/TextView - Task Name'), 0, FailureHandling.STOP_ON_FAILURE)

//Get Task Name
'Call Test Case Call Task Name'
Mobile.callTestCase(findTestCase('Test Cases/SleekFlow/Get Task Name'), null, FailureHandling.STOP_ON_FAILURE)

Mobile.verifyMatch(taskName, GlobalVariable.G_TaskName, false, FailureHandling.STOP_ON_FAILURE)






