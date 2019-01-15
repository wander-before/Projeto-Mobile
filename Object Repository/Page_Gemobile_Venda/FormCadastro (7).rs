<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>FormCadastro (7)</name>
   <tag></tag>
   <elementGuidId>f50a7562-0ac1-4279-bf76-62375ab13100</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='interface']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>interface</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>var venda = new Venda();venda.setModuloCliente('13', 'Cliente', 1)venda.mostrarSubMenusBlocoCadastro()var sys = new System();
    #lente {
        z-index: 500;
    }

    #janela{
        z-index: 550;
    }


    
        
            
                
                    Início
                
                
                    Venda
                
                
                                            Cadastro de Venda 
                                    
            
        

        
            
                                    Cadastro de Venda
                            

                            Venda de produtos e acessórios.
                    

        
            
                0
            
            
                R$
                0,00
            
        

        
            
                Passo 1 de 4
            
            
                
            
        
    



    
        

            
            
                
                
                
                
                
                                
                
                

                    

                        
                            Autenticação
                            
                                
                            
                        

                        

                            
                                
                                    Informe login e senha:
                                    
                                        E-mail: 
                                        
                                    
                                    
                                        Senha: 
                                        
                                    
                                
                            

                        

                        
                            Salvar
                        

                    

                

                

                    

                        
                            Autenticação
                            
                                
                            
                        

                        

                            
                        

                        
                            Salvar
                        

                    

                

                
                
                    
                        Atribua a venda a um vendedor
                    
                    

                                                    
                                
                                    Filial:
                                    
                                                                                
                                                                                            Escolha...
                                                                                                                                        Open - Filial
                                                                                            Open - Filial
                                                                                            Produtos Open
                                                                                    
                                    
                                
                                
                                    Vendedor:
                                    
                                        dropAutoComplete('us_id', '1', 'Vendedor', '14', '');
                                    
                                    
                                    
                                
                            
                                                
    if (venda == undefined) {
        var venda = new Venda();
    }
    var tem_erro = false;

    var fi_id = document.getElementById(&quot;fi_id&quot;);
    var us_id = document.getElementById(&quot;us_id&quot;);
    var cf_id = document.getElementById(&quot;cf_id&quot;);
    var tm_id = document.getElementById(&quot;tm_id&quot;);
    var us_id_aux = document.getElementById(&quot;us_id_aux&quot;);

    if (fi_id.value == &quot;&quot; || $(&quot;#fi_id option&quot;).size() > 1) {
        tem_erro = true;
    }

    if (us_id_aux.value == &quot;&quot;) {
        tem_erro = true;
    }

    if (us_id.value == &quot;&quot;) {
        tem_erro = true;
    }

    if ((venda.ehVisivel(cf_id) &amp;&amp; cf_id.value == &quot;&quot;) || $(&quot;#cf_id option&quot;).size() > 1) {
        tem_erro = true;
    }

    if ((venda.ehVisivel(tm_id) &amp;&amp; tm_id.value == &quot;&quot;) || $(&quot;#tm_id option&quot;).size() > 1) {
        tem_erro = true;
    }

    if (!tem_erro) {
        venda.avancar();
    }


                    
                
                
                
                
                                    
                        
                        
                        
                        
                            Buscar dados do Cliente
                        
                        
                            
                                
                                    
                                        
                                            
                                            Pessoa Física
                                        
                                        
                                            
                                            Pessoa Jurídica
                                        
                                        
                                            
                                            Estrangeiro
                                        
                                                                                    
                                                
                                                Celular
                                            
                                                                            
                                
                                
                                    CPF:
                                    
                                        
                                    
                                    
                                        
                                             Buscar
                                        
                                    
                                
                                
                                    CNPJ:
                                    
                                        
                                    
                                    
                                        
                                             Buscar
                                        
                                    
                                
                                
                                    RNE:
                                    
                                        
                                    
                                    
                                        
                                             Buscar
                                        
                                    
                                
                                                                    

                                        Celular:

                                        
                                            dropAutoComplete('celular_cliente', '1', 'ClienteCelular', '14', '');
                                        

                                        
                                            

                                            
                                        

                                    
                                                                
                                    
                                        
                                        
                                             Editar Cliente
                                        
                                    
                                
                                
                                     Score do Cliente: 
                                    
                                        
                                            Consulta não
                                                realizada
                                            
                                            Aprovado com
                                                restrição
                                            
                                            
                                                Aprovado
                                            
                                            Reprovado
                                            
                                        
                                    
                                
                                
                                    
                                         Nacionalidade: 
                                        
                                            
                                        
                                    
                                    
                                        Nome:
                                        
                                            
                                            
                                                 Cadastrar cliente
                                            
                                            
                                                 Editar Cliente
                                            
                                        
                                    
                                    
                                    Cliente cadastrado com sucesso
                                
                                
                            
                        
                    
                    
                                
                
                    
                        
                        
                                                    
                                
                                Produto
                                
                                    Por serial
                                    Por modelo
                                
                            
                        
                                                    
                                
                                Acessórios
                            
                        
                                            

                    
                        

    
        
            
                Produtos e Serviços
                
                    
                
            
            
                
                    
                        Aparelho/Serviços
                        
                            
                                IMEI:
                                
                                    Confirmar IMEI 
				  
			
			
			
			
			
			                                     Buscar Modelo
                                
                            
                        
                        
                            
                                Serviço:
                                
                                    Escolha ...                                
                            

                            
                                
                                    Escolha ...Troca de AparelhoMigração com Troca de AparelhoTroca de Simcard com Troca de AparelhoAtivação                                
                            

                            
                                                                
                                      Linha dependente 
                                

                                
                                      Conta Online 
                                
                            
                        
                        
                            
                                Novo Plano:

                                
                                    
                                        DDD11121314151617181921222427283132333435373841424344454647484951535455616263646566676869717374757779818283848586878889919293949596979899                                    
                                

                                
                                    
                                        Escolha ...                                    
                                

                            

                            

                                

                                    Valor Plano:
                                     0,00

                                

                                

                                    Tabela de Preço:

                                    
                                        Tabela 2018                                    

                                

                            

                        

                        
                            

                                
                                    Programa de Pontos:
                                    
                                        
                                        
                                    
                                

                            

                        

                        
                            
                                
                                    Pontos:
                                                                       

                                
                                    Valor desconto:
                                    $('#vc_pp_qtdeXXBotao').button();                                    
                            
                            
                                Descrição:
                                                               
                        

                        
                                                        
                                
                                    SVA:
                                    
                                        
                                        
                                    
                                
                            
                        
                        
                            
                                
                                    Valor Desconto: 
                                                                       
                            
                            
                                
                                                                    
                            
                        

                        

                        
                            
                                Plano Fidelizado:
                                
                                    
                                
                            
                                                        
                                
                                    Aparelho Fidelizado:
                                    
                                        
                                    
                                
                            


                            
                                
                                    Zerar remuneração da fidelização?
                                    
                                        
                                    
                                
                            

                            
                            
                            
                            
                        
                        
                            
                                Plano Antigo:
                                                                
                                    
                                        Escolha ...                                    
                                

                                
                                    
                                        Escolha ...                                    
                                
                            

                        
                        
                            
                                Débito automático:
                                
                                    
                                
                            
                        
                        
                            
                                
                                    Seguro:
                                    
                                        
                                        
                                    
                                
                            
                        
                        
                            
                                Indenização/Custo Mensal:
                                
                                    Escolha ...                                
                            
                        

                    

                    
                        Valores
                        
                            
                                Valor:
                                 - 

                                
                                    
                                
                            
                        
                        
                            
                                Valor Desconto: 
                                                               
                                                            
                                    Valor Adicional: 
                                    
                                
                                                        
                                Valor Garantia Estendida:
                                                               
                        

                        

                            

                                
                                    Vencimento da fatura: 
                                    
                                        EscolhaDia 01Dia 06Dia 10Dia 17Dia 21Dia 26                                    
                                

                                
                                    
                                    Cupom de desconto
                                

                            

                            
                                                                Cupom: 
                                
                                    
                                        
                                        Aplicar
                                        Remover
                                    
                                

                            

                            
                            

                        

                        
                    

                    
                        Linha

                        
                            
                                Número da linha:
                                
                                                               

                            
                                Número do Pedido Gemobile:
                                                                
                        
                        
                            
                                
                                     
                                    
                                         Portabilidade
                                    
                                
                                
                                     Número Portado: 
                                                                       
                            
                        

                                                    
                                
                                Nº da solicitação documental:
                                                                       
                            
                                                

                    
                        Simcard
                        
                            
                                ICCID Simcard:
                                
                                Confirmar ICCID 
				  
			
			
			
			
			
			                                
                            
                        

                        
                            
                                
                                     Cliente já possui simcard
                                    
                                    
                                
                            
                        

                        
                            
                                
                                    Valor Simcard:
                                     - 
                                    
                                

                            

                        

                        

                            

                                
                                    Desconto Chip:
                                                                       

                                
                                    
                                    Cupom de desconto
                                

                            

                            
                                                                Cupom: 
                                
                                    
                                        
                                         Aplicar 
                                         Remover 
                                    
                                

                            

                            
                            

                        
                    

                    
                        
                            Recarga na Ativação
                            
                                
                                    Com recarga na ativação:
                                    
                                        NãoRecarga Eletrônica                                    
                                

                                
                                    
                                          
                                        Com plano de dados -
                                         - R$ 0,00                                        
                                    
                                

                            
                            
                                
                                    Serial cartão:
                                    Confirmar Serial 
				  
			
			
			
			
			
			                                
                                
                                    Desconto Chip:
                                                                       
                            
                            
                                
                                    Valor:
                                    
                                        Escolha ...R$ 3,00R$ 5,00R$ 6,00R$ 7,00R$ 8,00R$ 10,00R$ 11,00R$ 12,00R$ 13,00R$ 14,00R$ 15,00R$ 16,00R$ 17,00R$ 18,00R$ 20,00R$ 21,00R$ 22,00R$ 25,00R$ 26,00R$ 27,00R$ 30,00R$ 35,00R$ 40,00R$ 50,00R$ 60,00R$ 100,00R$ 200,00R$ 300,00                                    
                                
                                
                                    Desconto Chip:
                                                                           
                            
                        
                    
                

            
            
                Salvar
            
        
    



0


    









                    
                    
                        Produtos e Serviços

                        
                            
                                                            
                        
                    

                    
                        

    
        
            
                Serviços
                
                    
                
            
            
                
                    
                        
                            
                                Serviço:
                                
                                    Escolha ...RecargaAtivaçãoTroca de PlanoTroca de SimcardMigraçãoSeguroSVA                                
                            
                        

                        
                            
                                Plano Novo:
                                
                                    DDD11121314151617181921222427283132333435373841424344454647484951535455616263646566676869717374757779818283848586878889919293949596979899                                
                            

                            
                                
                                    Escolha ...PréPósControle                                
                            
                        

                        
                            
                                Plano Avulso:
                                
                                    
                                        Escolha ...                                    
                                
                            
                        

                        
                            
                                
                                    Plano Antigo:
                                    
                                        Escolha ...PréPósControle                                    
                                
                            
                            
                                
                                    
                                        Escolha ...                                    
                                
                            
                        

                        

                            
                                Plano Fidelizado:
                                
                                    
                                
                            

                            
                                
                                     Conta Online
                                
                            

                            
                                
                                    
                                    Débito automático
                                
                            
                        

                        
                            
                                
                                    
						Cliente já possui simcard
					
					                                
                            
                            
                                
                                    ICCID:
                                    
                                        Confirmar ICCID 
				  
			
			
			
			
			
			                                    
                                
                            
                            
                                
                                    
                                        Valor do SIMCARD:
                                        
                                            ...
                                            
                                        
                                    
                                
                                
                                    
                                        Desconto Chip:
                                                                               
                                    
                                                                                    
                                            
                                                                                                                    
                                

                                

                                    
                                        
                                            
                                                
                                                Cupom de desconto
                                            
                                        
                                    

                                    
                                        

                                                                                        Cupom: 

                                            
                                                
                                                Aplicar
                                                 Remover 
                                            

                                        

                                    

                                    
                                    

                                

                            

                        
                        
                            
                                Número da linha: 
                                                               
                        

                        
                            
                                
                                     
                                    
                                         Portabilidade
                                    
                                

                                
                                     Número Portado: 
                                                                       
                            

                        
                        
                            
                                Nº da solicitação do Sistema GEMOBILE:
                                                               
                                                            
                                    Nº da solicitação documental:
                                                                       
                                                        

                        
                            
                                 Vencimento da fatura: 
                                
                                    EscolhaDia 01Dia 06Dia 10Dia 17Dia 21Dia 26                                
                            
                        

                    

                    
                        
                            Com recarga na ativação
                            
                                
                                    Dados Recarga
                                    
                                        NãoRecarga Eletrônica                                    
                                
                                
                                    
                                                
                                                      
                                                     Com plano de dados -
                                                     - R$ 0,00                                                    
                                                

                                    
                                
                            
                            
                                
                                    
                                        Serial:
                                        
			
			
			
			
			
			 #btConfirmarvs_ec_idXX{vertical-align:top; float: left; margin-top: 9px;}#btConfirmarvs_ec_idXX:hover{color:#000000;border:1px solid #696969;}Confirmar Serial  
				                                      

                                
                                
                                    
                                        Desconto Chip:
                                                                               
                                
                            


                            
                                
                                    Valor:
                                    
                                        Escolha ...R$ 3,00R$ 5,00R$ 6,00R$ 7,00R$ 8,00R$ 10,00R$ 11,00R$ 12,00R$ 13,00R$ 14,00R$ 15,00R$ 16,00R$ 17,00R$ 18,00R$ 20,00R$ 21,00R$ 22,00R$ 25,00R$ 26,00R$ 27,00R$ 30,00R$ 35,00R$ 40,00R$ 50,00R$ 60,00R$ 100,00R$ 200,00R$ 300,00                                    
                                
                                
                                    Desconto Chip:
                                                                       
                            

                        
                    

                
            
            
                Salvar
            
        
    



0

    









                    
                    
                        Serviços

                        
                            
                                                            
                        
                    

                    
                        0

	
		

            
                Acessórios
                
                    
                
            
            
                
                    
                        
                            Acessórios
                            
                                
                            
                            
                                
                                    Buscar Acessório
                            
                        
                    

                    
                         Acessórios disponíveis 
                        Nenhum produto encontrado!
                    

                    
                        
                            
                                
                                    Modelo:
                                    Xac_nomeX
                                

                                
                                    Código:
                                    Xac_codX
                                

                                
                                    Valor Unitário:
                                    
                                                                                0,00                                        
                                    
                                

                                
                                    Qtde disponível:
                                    XspanqtdeDisponivelX
                                
                            

                            
                                
                                    Quantidade:
                                    
                                

                                
                                    Valor desconto Unitário:
                                    
                                                                                
                                    
                                
                                                                    
                                        Valor Adicional Unitário:
                                        
                                        
                                    
                                                                
                                    Garantia Estendida:
                                    
                                

                            
                            

                                
                                    
                                        
                                            Cupom de desconto
                                        
                                        
                                    
                                

                                
                                    

                                                                                Cupom: 

                                        
                                            
                                            Aplicar
                                             Remover 
                                        

                                    

                                

                                
                                

                            
                            
                            
                                                    
                    
                
            
            
                Salvar
            
		
	
        
    







                    
                    
                        Acessórios

                        
                            
                                                            
                        
                    

                                            
                            Produto

                            
                                
                                    
0


    
        
            
                Buscar produto
                
                    
                
            
            
                
                    
                        
                            
                                Serial:
                                
                            

                            
                                Confirmar serial
                            
                        
                    
                
            
        
    

    
        

            
                Produto
                
                    
                
            

            
                

                    
                        
                            
                                Serial:
                                Xep_serialX
                            

                            
                                Descrição:
                                Xep_descricaoX
                            

                            
                                Valor:
                                Xvp_valorX
                            
                        

                        
                            
                                Valor Garantia Estendida:
                                                               

                                                            
                                    Valor Adicional:
                                    
                                                                    

                            

                        

                        

                            
                                Valor Desconto: 
                                                               

                            
                                

                                    
                                        
                                            Cupom de desconto
                                        
                                        
                                    

                                    

                                        
                                            Cupom: 
                                                                                       

                                        
                                            
                                            
                                        

                                    

                                
                            

                        
                        
                        
                        
                        
                        
                        

                                            

                
            
            
                Salvar
            
        
    

    		










                                                                    
                            
                        
                    
                    
                        0




    
        
            Recarga Eletrônica
            
                
            
        
        
            
                
                    
                        
                            
                                Valor:
                                
                                    Escolha ...R$ 3,00R$ 5,00R$ 6,00R$ 7,00R$ 8,00R$ 10,00R$ 11,00R$ 12,00R$ 13,00R$ 14,00R$ 15,00R$ 16,00R$ 17,00R$ 18,00R$ 20,00R$ 21,00R$ 22,00R$ 25,00R$ 26,00R$ 27,00R$ 30,00R$ 35,00R$ 40,00R$ 50,00R$ 60,00R$ 100,00R$ 200,00R$ 300,00                                
                            

                            
                                Desconto:
                                                               
                        

                        
                                                        
                                Quantidade:
                                
                            
                        

                        
                        
                            
                                
                                    
                                    Auto preenchimento
                                
                            
                            
                                
                                    Serial inicial:
                                                                     
                            
                        
                    
                    
                        
                            Adicionar
                        
                    
                
            
        
    




    
        
            Recarga
            
                
            
        

        
            
                                
                    

                        
                            
                                
                                    
                                        
                                            Valor:
                                            R$ Xrest_valorX
                                            
                                        
                                        
                                            Desconto:
                                            R$ Xrest_valor_descontoX
                                            
                                        
                                        
                                        
                                    
                                    
                                        
                                            Serial cartão:
                                            
			
			
			
			
			
			 #btConfirmarec_idXX{vertical-align:top; float: left; margin-top: 9px;}#btConfirmarec_idXX:hover{color:#000000;border:1px solid #696969;}Confirmar Serial  
				                                          
                                        
                                            Desconto:
                                                                                       
                                    
                                    
                                        
                                            Número da linha:
                                                                                       
                                        
                                            Código:
                                                                                       
                                        

                                            
                                                
                                                    Cupom de desconto
                                                
                                                
                                            

                                        
                                        
                                            

                                                                                                Cupom: 

                                                
                                                    
                                                    Aplicar
                                                     Remover 
                                                

                                            

                                            
                                            

                                        
                                    
                                
                            
                            
                        
                    
                
        
        
            Salvar
        
    












                    
                    
                        Recarga

                        
                            
                                                            
                        
                    
                
                

                
                
                    
                        
                        0


OPÇÕES DE PAGAMENTO PARA INSERIR 

    
        
            
        
        Dinheiro
        
    
    
        
            
        
        Cartão
    
    
        
            
        
        Outros
    
    
        
            
        
        Pagamento em Lote
    



    
                    
                
                    Forma de pagamento
                    
                        Escolha ...DinheiroChequeCheque PréBoletoProdutoPromissória                    
                
                
                    
                
                
                    parcelado em
                    
                        1x2x3x4x5x6x7x8x9x10x11x12x13x14x15x16x17x18x19x20x21x22x23x24x25x26x27x28x29x30x31x32x33x34x35x36x37x38x39x40x41x42x43x44x45x46x47x48x                    
                
                
                     Adicionar em lote 
                
            
            
                Obs: Serão excluídas todas as parcelas já adicionadas.
            
            



    
         Adicionar Pagamento
    




    
        
        
            
                Cartão
                
                    
                    
                
            
            
                
                    
                        
                            Tipo:
                            
                                Escolha...Cartão CréditoCartão Débito                            
                            
                                                    
                    
                    
                        
                            
                                Valor:
                                                               
                            
                                Parcelado em:
                                
                                    1x2x3x4x5x6x7x8x9x10x11x12x13x14x15x16x17x18x19x20x21x22x23x24x25x26x27x28x29x30x31x32x33x34x35x36x37x38x39x40x41x42x43x44x45x46x47x48x                                
                            
                        
                        
                            Nº de autorização cartão:
                                                       
                    
                    
                        
                            Observações:
                            
                        
                    
                
            
        
    

    
        
            
                Dinheiro
                
                    
                    
                
            
            
                
                
                    
                        
                            Valor:
                                                       
                    
                    
                        
                            Observações:
                            
                        
                    
                
            
        
    

    
        
            
                Outros
                
                    
                    
                
            
            
                
                    
                        
                            Forma de pagamento:
                            
                                Escolha...DinheiroChequeCheque PréBoletoProdutoPromissória                                
                            
                        
                    
                    
                        
                             Informações do aparelho 
                            
                                
                                    Movimentação:
                                    
                                        Devolução na VendaCompra na venda                                        
                                    
                                
                                
                                    Deseja efetivar o cancelamento de comissão e estorno?
                                        
                                        
                                            
			
				$(document).ready(function() {

					$('#cancela-comissaoXXInfo').qtip({
						content: {
							title: 'Observação'
						},
						position: {
							my: 'bottom left',
							at: 'top right'
						},
						style: {
								classes: 'qtip-light qtip-rounded qtip-bootstrap',
								tip: true, // Apply a speech bubble tip to the tooltip at the designated tooltip corner
								border: {
									width: 0,
									radius: 4,
									color: '#333'
								},
								title: {
									'font-size': 11
								},
								width: { min: 100, max: 100 } //largura mínima e máxima
						},
						show: {
							event: 'click mouseenter'
						}
					 });
				});
			
		
								Sim
								Não
			
			                                        
                                    
                                    
                                
                                
                                    Produto:
                                    
                                        AcessórioProduto                                    
                                
                                
                                    
                                        
                                            
                                                
                                                    Modelo:
                                                    
																	
																		
																				
																							

																		
																				
																							
																		
																			SimCard/Micro/Nano 4GSimCard/Micro/Nano Fixo 4GSimCard/Micro/Nano Pré																		
																	
                                                        
																		Será usado o modelo que está cadastrado no sistema.
																	
                                                    
                                                
                                                
                                                    Serial:
                                                    
                                                                                                               
                                                
                                                
                                                    Produto Referente a Devolução:
                                                    
                                                        
                                                            Nenhum
                                                        
                                                    
                                                
                                            
                                        

                                        
                                            
                                                
                                                    Modelo:
                                                    
                                                        	
					                                                    
                                                
                                            
                                        

                                    
                                
                            
                        
                        

                                            
                    
                        
                            
                                Data de Vencimento:
                                                               

                            
                                Valor da parcela:
                                                               
                        
                        
                            Nº do Documento:
                                                       
                    
                    
                        
                            Observações:
                            
                        
                    
                
            
        
    

    
        
            
                Outros
                
                    
                    
                
            
            
                
                    
                        
                            Forma de pagamento:
                            
                                Escolha...DinheiroChequeCheque PréBoletoProdutoPromissória                                
                            
                        
                    
                    
                        
                             Informações do aparelho 
                            
                                
                                    Movimentação:
                                    
                                        Devolução na VendaCompra na venda                                        
                                    
                                
                                
                                    Deseja efetivar o cancelamento de comissão e estorno?
                                        
                                        
                                            
			
				$(document).ready(function() {

					$('#cancela-comissaoXXInfo').qtip({
						content: {
							title: 'Observação'
						},
						position: {
							my: 'bottom left',
							at: 'top right'
						},
						style: {
								classes: 'qtip-light qtip-rounded qtip-bootstrap',
								tip: true, // Apply a speech bubble tip to the tooltip at the designated tooltip corner
								border: {
									width: 0,
									radius: 4,
									color: '#333'
								},
								title: {
									'font-size': 11
								},
								width: { min: 100, max: 100 } //largura mínima e máxima
						},
						show: {
							event: 'click mouseenter'
						}
					 });
				});
			
		
								Sim
								Não
			
			                                        
                                    
                                    
                                
                                
                                    Produto:
                                    
                                        AcessórioProduto                                    
                                
                                
                                    
                                        
                                            
                                                
                                                    Modelo:
                                                    
																	
																		
																				
																							

																		
																				
																							
																		
																			SimCard/Micro/Nano 4GSimCard/Micro/Nano Fixo 4GSimCard/Micro/Nano Pré																		
																	
                                                        
																		Será usado o modelo que está cadastrado no sistema.
																	
                                                    
                                                
                                                
                                                    Serial:
                                                    
                                                                                                               
                                                
                                                
                                                    Produto Referente a Devolução:
                                                    
                                                        
                                                            Nenhum
                                                        
                                                    
                                                
                                            
                                        

                                        
                                            
                                                
                                                    Modelo:
                                                    
                                                        	
					                                                    
                                                
                                            
                                        

                                    
                                
                            
                        
                        

                                            
                    
                        
                            
                                Data de Vencimento:
                                                               

                            
                                Valor da parcela:
                                                               
                        
                        
                            Nº do Documento:
                                                       
                    
                    
                        
                            Observações:
                            
                        
                    
                
            
        
    
    

    








                    
                
                

                
                    Voltar
                    Avançar

                    
                        Salvar                    
                
                
                
            
        
    













    venda.mostrarContainersVenda();

    $(window).ready(function () {
        $(&quot;#spantotalVenda&quot;).html('0,00');
        venda.totalItens();
    });

    $(document).ready(function () {
        atualizaTotal();
        $(&quot;#fi_id&quot;).change();
    });

    /**
     * @param {type} String
     * @returns {Boolean}
     *
     * n[0] = Não validada
     * n[1] = Validada
     * n[2] = Tem folha de pagamento
     * n[3] = Não tem folha de pagamento
     */
    function validaAlteracaoVenda(result) {
        if (result == undefined) {
            var valores = $('#form20').serialize();
            _makeRequestCallback('ajaxVenda.php', valores, validaAlteracaoVenda);
            return false;
        } else {
            var n = result.slice(-1);

            var input = document.createElement('input');
            input.setAttribute('id', 'hiddenaux');
            input.setAttribute('name', 'hiddenaux');
            input.setAttribute('type', 'hidden');

            switch (n) {
                case &quot;1&quot;:
                    $(&quot;#form20&quot;).submit();
                    break;
                case &quot;2&quot;:
                case &quot;4&quot;:
                    $(function () {
                        $(&quot;div[class=\&quot;ui-dialog-title\&quot;]:contains(\&quot;Empty the recycle bin?\&quot;)&quot;).html(&quot;Folha de pagamento&quot;);
                        $(&quot;#dialog-confirm p&quot;).html(&quot;&lt;span class=\&quot;ui-icon ui-icon-alert\&quot; style=\&quot;float:left; margin:0 7px 10px 0;\&quot;>&lt;/span>A folha de pagamento deste mês para este colaborador já foi gerada.&lt;/br>&lt;/br>O que deseja fazer?&quot;);

                        if (n == &quot;2&quot;) {
                            $(&quot;#dialog-confirm&quot;).dialog({
                                resizable: false,
                                height: 210,
                                width: 340,
                                modal: true,
                                buttons: {
                                    &quot;Salvar e excluir a folha&quot;: function () {
                                        input.setAttribute('value', &quot;1&quot;);
                                        document.getElementById(&quot;form20&quot;).appendChild(input);

                                        $(&quot;#dialog-confirm p&quot;).html(&quot;&quot;);

                                        $(this).dialog(&quot;close&quot;);

                                        $(&quot;#form20&quot;).submit();
                                    },
                                    Cancelar: function () {
                                        $(&quot;#dialog-confirm p&quot;).html(&quot;&quot;);
                                        $(this).dialog(&quot;close&quot;);
                                    }
                                }
                            });
                        } else if (n == &quot;4&quot;) {
                            $(&quot;#dialog-confirm&quot;).dialog({
                                resizable: false,
                                height: 210,
                                width: 440,
                                modal: true,
                                buttons: {
                                    &quot;Salvar e manter a folha&quot;: function () {
                                        input.setAttribute('value', &quot;0&quot;);
                                        document.getElementById(&quot;form20&quot;).appendChild(input);

                                        $(&quot;#dialog-confirm p&quot;).html(&quot;&quot;);

                                        $(this).dialog(&quot;close&quot;);

                                        $(&quot;#form20&quot;).submit();
                                    },
                                    &quot;Salvar e excluir a folha&quot;: function () {
                                        input.setAttribute('value', &quot;1&quot;);
                                        document.getElementById(&quot;form20&quot;).appendChild(input);

                                        $(&quot;#dialog-confirm p&quot;).html(&quot;&quot;);

                                        $(this).dialog(&quot;close&quot;);

                                        $(&quot;#form20&quot;).submit();
                                    },
                                    Cancelar: function () {
                                        $(&quot;#dialog-confirm p&quot;).html(&quot;&quot;);
                                        $(this).dialog(&quot;close&quot;);
                                    }
                                }
                            });
                        }
                    });
                    break;
                case &quot;3&quot;:
                    $(function () {
                        $(&quot;#dialog-message p&quot;).html(&quot;&lt;span class=\&quot;ui-icon ui-icon-alert\&quot; style=\&quot;float:left; margin:0 7px 50px 0;\&quot;>&lt;/span>Não é possível realizar o lançamento, porque a folha de pagamento já foi gerada.&quot;);
                        $(&quot;#dialog-message&quot;).dialog({
                            modal: true,
                            buttons: {
                                Ok: function () {
                                    $(&quot;#dialog-message p&quot;).html(&quot;&quot;);
                                    $(this).dialog(&quot;close&quot;);
                                }
                            }
                        });
                    });
                    break;
                default :
                    $(&quot;#form20&quot;).submit();
                    break;

            }
            return false;
        }
    }

</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;interface&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <value>//div[@id='interface']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <value>//div[@id='interfaceGeral']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Relatórios'])[5]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Valores'])[1]/following::div[2]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <value>//div[2]/div</value>
   </webElementXpaths>
</WebElementEntity>
