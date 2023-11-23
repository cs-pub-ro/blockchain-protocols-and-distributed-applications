"use strict";(self.webpackChunkblockchain_protocols_and_distributed_applications=self.webpackChunkblockchain_protocols_and_distributed_applications||[]).push([[1241],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>h});var a=r(7294);function n(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,a)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){n(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,a,n=function(e,t){if(null==e)return{};var r,a,n={},o=Object.keys(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||(n[r]=e[r]);return n}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(a=0;a<o.length;a++)r=o[a],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(n[r]=e[r])}return n}var c=a.createContext({}),l=function(e){var t=a.useContext(c),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},p=function(e){var t=l(e.components);return a.createElement(c.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},m=a.forwardRef((function(e,t){var r=e.components,n=e.mdxType,o=e.originalType,c=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),u=l(r),m=n,h=u["".concat(c,".").concat(m)]||u[m]||d[m]||o;return r?a.createElement(h,i(i({ref:t},p),{},{components:r})):a.createElement(h,i({ref:t},p))}));function h(e,t){var r=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var o=r.length,i=new Array(o);i[0]=m;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s[u]="string"==typeof e?e:n,i[1]=s;for(var l=2;l<o;l++)i[l]=r[l];return a.createElement.apply(null,i)}return a.createElement.apply(null,r)}m.displayName="MDXCreateElement"},2147:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>c,contentTitle:()=>i,default:()=>d,frontMatter:()=>o,metadata:()=>s,toc:()=>l});var a=r(7462),n=(r(7294),r(3905));const o={},i="Never Sea Festival Smart Contract",s={unversionedId:"Practical Sessions/Smart Contracts/sc",id:"Practical Sessions/Smart Contracts/sc",title:"Never Sea Festival Smart Contract",description:"You are the Never Sea Festival 2024 organizers and you decide to create the registration via blockchain.",source:"@site/docs/Practical Sessions/Smart Contracts/sc.md",sourceDirName:"Practical Sessions/Smart Contracts",slug:"/Practical Sessions/Smart Contracts/sc",permalink:"/blockchain-protocols-and-distributed-applications/Practical Sessions/Smart Contracts/sc",draft:!1,tags:[],version:"current",frontMatter:{},sidebar:"sidebar",previous:{title:"Smart Contracts",permalink:"/blockchain-protocols-and-distributed-applications/Practical Sessions/Smart Contracts/"},next:{title:"Assignments",permalink:"/blockchain-protocols-and-distributed-applications/Assignments/"}},c={},l=[{value:"Prerequisites",id:"prerequisites",level:2},{value:"Compile and deploy the Smart Contract template",id:"compile-and-deploy-the-smart-contract-template",level:2},{value:"Practice",id:"practice",level:2}],p={toc:l},u="wrapper";function d(e){let{components:t,...r}=e;return(0,n.kt)(u,(0,a.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,n.kt)("h1",{id:"never-sea-festival-smart-contract"},"Never Sea Festival Smart Contract"),(0,n.kt)("p",null,"You are the Never Sea Festival 2024 organizers and you decide to create the registration via blockchain.\nStarting from Smart Contract template you have to add more features to coordinate the event."),(0,n.kt)("h2",{id:"prerequisites"},"Prerequisites"),(0,n.kt)("p",null,"You need to have mxpy installed. Follow the installation guide ",(0,n.kt)("a",{parentName:"p",href:"https://docs.multiversx.com/sdk-and-tools/sdk-py/installing-mxpy/"},"here"),"."),(0,n.kt)("p",null,"Clone the ",(0,n.kt)("a",{parentName:"p",href:"https://github.com/systems-cs-pub-ro/Foundation-Of-Blockchains/tree/master/labs/lab03/Neversea"},"Neversea")," project."),(0,n.kt)("h2",{id:"compile-and-deploy-the-smart-contract-template"},"Compile and deploy the Smart Contract template"),(0,n.kt)("p",null,"Use ",(0,n.kt)("a",{parentName:"p",href:"https://docs.multiversx.com/developers/tutorials/counter/#build-the-contract"},"tutorial")," to build the smart contract."),(0,n.kt)("p",null,"To check that the contract was ",(0,n.kt)("strong",{parentName:"p"},"successfully built"),", verify that there was a ",(0,n.kt)("strong",{parentName:"p"},"wasm")," (WebAssembly) file generate: ",(0,n.kt)("strong",{parentName:"p"},"output/neversea.wasm"),". This is the compiled code of your contract."),(0,n.kt)("p",null,"To check that the contract was successfully deployed, check the devnet/testnet."),(0,n.kt)("hr",null),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"NOTE")),(0,n.kt)("p",null,"Check the deployment on the explorer. Do not assume that the contract was successfully deployed if there are no command line errors."),(0,n.kt)("p",null,"Any modification of the contract must be succeeded by a compilation and deployment!"),(0,n.kt)("hr",null),(0,n.kt)("h2",{id:"practice"},"Practice"),(0,n.kt)("ul",null,(0,n.kt)("li",{parentName:"ul"},"Make a contract call to register a user;"),(0,n.kt)("li",{parentName:"ul"},"Make a contract call to view the registered users;"),(0,n.kt)("li",{parentName:"ul"},"Modify the registration endpoint to enable VIP access;"),(0,n.kt)("li",{parentName:"ul"},"Create a new storage mapper ",(0,n.kt)("strong",{parentName:"li"},"registration_fee_vip"),";"),(0,n.kt)("li",{parentName:"ul"},"Create a new storage mapper ",(0,n.kt)("strong",{parentName:"li"},"vip_participants")," to save the VIP participants;"),(0,n.kt)("li",{parentName:"ul"},"In the registration endpoint, make a verification of the tokens received. If the tokens received is ",(0,n.kt)("strong",{parentName:"li"},"registration_fee_vip"),", add the user to ",(0,n.kt)("strong",{parentName:"li"},"vip_participants"),", if the amount is ",(0,n.kt)("strong",{parentName:"li"},"registration_fee"),", add them to participants, else, deny registration;"),(0,n.kt)("li",{parentName:"ul"},"Modify the registration fee to enable Early Bird and Full price access;"),(0,n.kt)("li",{parentName:"ul"},"Create a new endpoint that modifies the ",(0,n.kt)("strong",{parentName:"li"},"registration_fee")," and ",(0,n.kt)("strong",{parentName:"li"},"registration_fee_vip")," storage mapper. This endpoint should be call ",(0,n.kt)("strong",{parentName:"li"},"only by the owner"),"."),(0,n.kt)("li",{parentName:"ul"},"BONUS: Create a feature to enable 50% discount vouchers for friends and partners. Create a list of hardcoded discount codes. Create a new endpoint that receives a discount code as a parameter and registers a user with 50% discount.")),(0,n.kt)("hr",null),(0,n.kt)("p",null,(0,n.kt)("strong",{parentName:"p"},"Hint"),"\nUse #","[only_owner]"," endpoint annotation."),(0,n.kt)("hr",null))}d.isMDXComponent=!0}}]);