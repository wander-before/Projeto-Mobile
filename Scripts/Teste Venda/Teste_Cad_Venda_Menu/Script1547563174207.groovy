import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('Teste Tela de Login/Teste Login Valido'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Gemobile_Venda/MenuVenda'))

WebUI.click(findTestObject('Page_Gemobile_Venda/SubMenuVenda'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Op_Venda'))

WebUI.click(findTestObject('Page_Gemobile_Venda/InserirNovaVenda'))

WebUI.waitForElementVisible(findTestObject('Page_Gemobile_Venda/FormCadastro'), 0)

WebUI.selectOptionByValue(findTestObject('Page_Gemobile_Venda/Select_EscolhaFilial'), '39', false)

WebUI.delay(1)

WebUI.selectOptionByValue(findTestObject('Page_Gemobile_Venda/Select_Escolha...Caixa'), '1', false)

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_AvancaCad'))

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/SelecionarOpDeCliente'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/SelecionaCelular'))

WebUI.setText(findTestObject('Page_Gemobile_Venda/SetCampoCelular'), '0000')

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/Esperar_ul_00 00000-0000 - Teste Katal'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/li_00 00000-0000 - Teste Katal'))

WebUI.click(findTestObject('Page_Gemobile_Venda/FormClienteClick'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_AvancaCad (1)'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/OpProduto'))

WebUI.click(findTestObject('Page_Gemobile_Venda/OpProdPorModelo'))

WebUI.setText(findTestObject('Page_Gemobile_Venda/SetCampoModelo'), 'XT171')

WebUI.delay(2, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Gemobile_Venda/li_Teste XT1710 Branco MOTOROL1'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_Buscar'))

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/imgConfirmar'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/imgConfirmar'))

WebUI.acceptAlert()

WebUI.delay(2)

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_SalvarProduto (1)'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_AvancaCad'))

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/ModalPagamento (1)'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/OpPag_Dinheiro'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_AddPag'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_SalvarPag'))

WebUI.waitForAlert(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Fechar_Alert'), FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Page_Gemobile_TelaLogin/SetCampoLogin'), 'proprietário.open')

WebUI.setText(findTestObject('Page_Gemobile_TelaLogin/SetCampoSenha'), 'venus2018')

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_SalvarAuten'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Bt_SalvarPag'))

WebUI.verifyElementPresent(findTestObject('Page_Gemobile_Venda/Alert_VendaInseridaComSucesso'), 0)

