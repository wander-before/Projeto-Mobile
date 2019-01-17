<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>OpAcoes</name>
   <tag></tag>
   <elementGuidId>ef07fa38-b59b-4809-9970-5abea622334d</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//form[@id='formVenda']/table/tbody/tr/td[9]/div</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>dropdown-button</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>Editar a vendaConfirmar venda
                                 $( document ).ready(function() {
                                    var bt = $('#btDelVenda994');
                                    bt.qtip(
                                    {
                                        content: {
                                        // Set the text to an image HTML string with the correct src URL to the loading image you want to use
                                            ajax: {
                                                url: bt.attr('data-url'),
                                                loading: false
                                            },
                                            title: {
                                                text: bt.attr('title'), // Give the tooltip a title using each elements text
                                                button: 'X' // Show a close link in the title
                                            }
                                        },
                                        position: {
                                            target: bt,
                                            adjust: {
                                                x:-750,
                                                y:-123,
                                                screen: false // Keep the tooltip on-screen at all times
                                            }
                                        },
                                        show: {
                                            event: 'click',
                                            solo: true // Only show one tooltip at a time
                                        },
                                        hide: {
                                            event: 'none'
                                        },
                                        style: {
                                            classes: 'qtip-light qtip-rounded qtip-bootstrap',
                                            tip: 'right center'
                                        }
                                    })});
                                Cancelar venda </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;formVenda&quot;)/table[@class=&quot;expense-table&quot;]/tbody[1]/tr[@class=&quot;expense-table--delayed&quot;]/td[9]/div[@class=&quot;dropdown-button&quot;]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <value>//form[@id='formVenda']/table/tbody/tr/td[9]/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Teste Katalon23'])[1]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Proprietário Open'])[2]/following::div[1]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <value>//td[9]/div</value>
   </webElementXpaths>
</WebElementEntity>
