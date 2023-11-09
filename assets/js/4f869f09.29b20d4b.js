"use strict";(self.webpackChunkblockchain_protocols_and_distributed_applications=self.webpackChunkblockchain_protocols_and_distributed_applications||[]).push([[21],{3905:(e,t,n)=>{n.d(t,{Zo:()=>p,kt:()=>m});var a=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function o(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function i(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?o(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},o=Object.keys(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)n=o[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var s=a.createContext({}),c=function(e){var t=a.useContext(s),n=t;return e&&(n="function"==typeof e?e(t):i(i({},t),e)),n},p=function(e){var t=c(e.components);return a.createElement(s.Provider,{value:t},e.children)},u="mdxType",k={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},d=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,o=e.originalType,s=e.parentName,p=l(e,["components","mdxType","originalType","parentName"]),u=c(n),d=r,m=u["".concat(s,".").concat(d)]||u[d]||k[d]||o;return n?a.createElement(m,i(i({ref:t},p),{},{components:n})):a.createElement(m,i({ref:t},p))}));function m(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var o=n.length,i=new Array(o);i[0]=d;var l={};for(var s in t)hasOwnProperty.call(t,s)&&(l[s]=t[s]);l.originalType=e,l[u]="string"==typeof e?e:r,i[1]=l;for(var c=2;c<o;c++)i[c]=n[c];return a.createElement.apply(null,i)}return a.createElement.apply(null,n)}d.displayName="MDXCreateElement"},4545:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>k,frontMatter:()=>o,metadata:()=>l,toc:()=>c});var a=n(7462),r=(n(7294),n(3905));const o={},i="Mint tokens",l={unversionedId:"Practical Sessions/Money/mint_tokens",id:"Practical Sessions/Money/mint_tokens",title:"Mint tokens",description:"In this section you will learn how to mint tokens on MultiversX.",source:"@site/docs/Practical Sessions/Money/mint_tokens.md",sourceDirName:"Practical Sessions/Money",slug:"/Practical Sessions/Money/mint_tokens",permalink:"/blockchain-protocols-and-distributed-applications/Practical Sessions/Money/mint_tokens",draft:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Money",permalink:"/blockchain-protocols-and-distributed-applications/Practical Sessions/Money/"},next:{title:"Swap tokens on xExchange",permalink:"/blockchain-protocols-and-distributed-applications/Practical Sessions/Money/swap"}},s={},c=[{value:"Mint ESDT",id:"mint-esdt",level:2},{value:"Practice",id:"practice",level:2}],p={toc:c},u="wrapper";function k(e){let{components:t,...o}=e;return(0,r.kt)(u,(0,a.Z)({},p,o,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h1",{id:"mint-tokens"},"Mint tokens"),(0,r.kt)("p",null,"In this section you will learn how to mint tokens on MultiversX."),(0,r.kt)("p",null,"There are 2 types of tokens on MultiversX:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Native tokens - EGLD;"),(0,r.kt)("li",{parentName:"ul"},"ESDT - eStandard Digital Token.")),(0,r.kt)("p",null,"In the previous section you learnt how to mint xEGLD on ",(0,r.kt)("a",{parentName:"p",href:"https://testnet-wallet.multiversx.com/"},"MultiversX Testnet")," using ",(0,r.kt)("inlineCode",{parentName:"p"},"Faucet")," option."),(0,r.kt)("p",null,(0,r.kt)("img",{alt:"Testnet Wallet",src:n(603).Z,width:"1913",height:"876"})),(0,r.kt)("h2",{id:"mint-esdt"},"Mint ESDT"),(0,r.kt)("p",null,"This time we use the ",(0,r.kt)("inlineCode",{parentName:"p"},"Create Token")," option."),(0,r.kt)("p",null,"Token Name:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"length between 3 and 20 characters"),(0,r.kt)("li",{parentName:"ul"},"alphanumeric characters only")),(0,r.kt)("p",null,"Token Ticker:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"length between 3 and 10 characters"),(0,r.kt)("li",{parentName:"ul"},"alphanumeric UPPERCASE only")),(0,r.kt)("p",null,"Number of decimals:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"should be a numerical value between 0 and 18;"),(0,r.kt)("li",{parentName:"ul"},"there are no floats on the blockchain;"),(0,r.kt)("li",{parentName:"ul"},"a token with 3 decimals and value 1000 would be equal with value 1;"),(0,r.kt)("li",{parentName:"ul"},"EGLD has 18 decimals and the value of 1000000000000000000 is 1 EGLD.")),(0,r.kt)("p",null,"Let's create a token new token:"),(0,r.kt)("p",null,(0,r.kt)("img",{alt:"Issue Token",src:n(6501).Z,width:"1913",height:"876"})),(0,r.kt)("p",null,"and inspect the transaction "),(0,r.kt)("p",null,(0,r.kt)("img",{alt:"Explorer Token Created",src:n(730).Z,width:"1913",height:"876"})),(0,r.kt)("p",null,"Observe ",(0,r.kt)("inlineCode",{parentName:"p"},"Token Operations")," field. We received ",(0,r.kt)("inlineCode",{parentName:"p"},"321.00 BPDA-208994")," tokens. ",(0,r.kt)("inlineCode",{parentName:"p"},"BPDA-208994")," is the ",(0,r.kt)("strong",{parentName:"p"},"token ID"),". The token ID is formed by appending to the token ticker the character ",(0,r.kt)("strong",{parentName:"p"},"-")," and 6 random hexadecimals characters. This is done because there might be mutiple tokens with the same ticker; the token ID is always unique."),(0,r.kt)("p",null,"We can click on the token ID and see the details of the token:"),(0,r.kt)("p",null,(0,r.kt)("img",{alt:"Token Details",src:n(8719).Z,width:"1913",height:"876"})),(0,r.kt)("p",null,"Observe the fields ",(0,r.kt)("strong",{parentName:"p"},"TOKEN"),", ",(0,r.kt)("strong",{parentName:"p"},"Supply"),", ",(0,r.kt)("strong",{parentName:"p"},"Holders"),", ",(0,r.kt)("strong",{parentName:"p"},"Transactions"),", ",(0,r.kt)("strong",{parentName:"p"},"Owner"),", ",(0,r.kt)("strong",{parentName:"p"},"Decimals"),"."),(0,r.kt)("p",null,"Let create another token with the same input:"),(0,r.kt)("p",null,(0,r.kt)("img",{alt:"Issue Token2",src:n(8837).Z,width:"1913",height:"876"})),(0,r.kt)("p",null,"Observe that the token ID is different."),(0,r.kt)("h2",{id:"practice"},"Practice"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Create your own ESDT token;"),(0,r.kt)("li",{parentName:"ul"},"Inspect the transaction;"),(0,r.kt)("li",{parentName:"ul"},"Send some of your tokens to ",(0,r.kt)("strong",{parentName:"li"},"erd1ld6er5zpdze3cynzkapur9qhzh826jje6n87g7tvdfrtszs8jn2qv44nqd"),";"),(0,r.kt)("li",{parentName:"ul"},"Create another token with the same ticker. Observer that the token ID is different and unique.")))}k.isMDXComponent=!0},730:(e,t,n)=>{n.d(t,{Z:()=>a});const a=n.p+"assets/images/explorer_token_created-ded31099fc5973ca1a9ec7d7e0d399be.png"},6501:(e,t,n)=>{n.d(t,{Z:()=>a});const a=n.p+"assets/images/issue_token-3e761069bd1f5df2f41ec5f2454500f4.png"},8837:(e,t,n)=>{n.d(t,{Z:()=>a});const a=n.p+"assets/images/issue_token2-a41a1e04bcf1885d59aa1d95d843e6bd.png"},603:(e,t,n)=>{n.d(t,{Z:()=>a});const a=n.p+"assets/images/testnet_wallet-7576edfc7a5e57b18b9674533717b18d.png"},8719:(e,t,n)=>{n.d(t,{Z:()=>a});const a=n.p+"assets/images/token_details-d263d3ea45c0d2d627422cf0da9607dc.png"}}]);