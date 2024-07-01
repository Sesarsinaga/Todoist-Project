import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import org.eclipse.persistence.internal.oxm.record.deferred.SkippedEntityEvent
import org.testng.SkipException

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject

import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile

import internal.GlobalVariable as GlobalVariable

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import io.appium.java_client.AppiumDriver

class DigiposTestListener {
	
	def isTestSuite = false 
	
	@BeforeTestCase
	def beforeTestCase(TestCaseContext testCaseContext) {
		   if(!isTestSuite){
			   Mobile.startExistingApplication('com.todoist')
			   Mobile.delay(5)
	   }
	}
	
	@AfterTestCase
	def afterTestCase(TestCaseContext testCaseContext) {
		if(!isTestSuite){
		   try {
			   AppiumDriver<?> driver = MobileDriverFactory.getDriver();
			   driver.terminateApp('com.todoist')
			} catch(Exception ex) {
			   println("Catching the exception");
			}
		
	   }
	}
}