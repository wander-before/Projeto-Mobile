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

WebUI.click(findTestObject('Page_Gemobile_Venda/Page Cancelar Venda/MenuVenda'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/SubMenuVenda'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page Cancelar Venda/Op_Venda'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/InserirNovaVenda'))

WebUI.waitForElementVisible(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/FormCadastro'), 0)

WebUI.selectOptionByValue(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Select_EscolhaFilial'), '39', false)

WebUI.delay(1)

WebUI.selectOptionByValue(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Select_Escolha...Caixa'), '1', false)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_AvancaCad'))

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/SelecionarOpDeCliente'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/SelecionaCelular'))

WebUI.setText(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/SetCampoCelular'), '0000')

WebUI.delay(2, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/li_00 00000-0000 - Teste Katal'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/FormClienteClick'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_AvancaCad'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/OpProduto'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/OpProdPorModelo'))

WebUI.setText(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/SetCampoModelo'), 'XT171')

WebUI.delay(2, FailureHandling.STOP_ON_FAILURE)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/li_Teste XT1710 Branco MOTOROL1'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_Buscar'))

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/imgConfirmar'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/imgConfirmar'))

WebUI.acceptAlert()

WebUI.delay(2)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_SalvaProduto'))

WebUI.delay(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_AvancaCad'))

WebUI.waitForElementPresent(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/ModalPagamento'), 0)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/OpPag_Dinheiro'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_AddPag'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_SalvarPag'))

WebUI.waitForAlert(1)

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Fechar_Alert'), FailureHandling.STOP_ON_FAILURE)

WebUI.setText(findTestObject('Page_Gemobile_TelaLogin/SetCampoLogin'), 'proprietário.open')

WebUI.setText(findTestObject('Page_Gemobile_TelaLogin/SetCampoSenha'), 'venus2018')

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_SalvarAuten'))

WebUI.click(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Bt_SalvarPag'))

WebUI.verifyElementPresent(findTestObject('Page_Gemobile_Venda/Page VendaMenu_VendaBotao/Alert_VendaInseridaComSucesso'), 
    0)

