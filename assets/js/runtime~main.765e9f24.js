(()=>{"use strict";var e,a,t,d,r,f={},c={};function o(e){var a=c[e];if(void 0!==a)return a.exports;var t=c[e]={id:e,loaded:!1,exports:{}};return f[e].call(t.exports,t,t.exports,o),t.loaded=!0,t.exports}o.m=f,o.c=c,e=[],o.O=(a,t,d,r)=>{if(!t){var f=1/0;for(i=0;i<e.length;i++){t=e[i][0],d=e[i][1],r=e[i][2];for(var c=!0,b=0;b<t.length;b++)(!1&r||f>=r)&&Object.keys(o.O).every((e=>o.O[e](t[b])))?t.splice(b--,1):(c=!1,r<f&&(f=r));if(c){e.splice(i--,1);var n=d();void 0!==n&&(a=n)}}return a}r=r||0;for(var i=e.length;i>0&&e[i-1][2]>r;i--)e[i]=e[i-1];e[i]=[t,d,r]},o.n=e=>{var a=e&&e.__esModule?()=>e.default:()=>e;return o.d(a,{a:a}),a},t=Object.getPrototypeOf?e=>Object.getPrototypeOf(e):e=>e.__proto__,o.t=function(e,d){if(1&d&&(e=this(e)),8&d)return e;if("object"==typeof e&&e){if(4&d&&e.__esModule)return e;if(16&d&&"function"==typeof e.then)return e}var r=Object.create(null);o.r(r);var f={};a=a||[null,t({}),t([]),t(t)];for(var c=2&d&&e;"object"==typeof c&&!~a.indexOf(c);c=t(c))Object.getOwnPropertyNames(c).forEach((a=>f[a]=()=>e[a]));return f.default=()=>e,o.d(r,f),r},o.d=(e,a)=>{for(var t in a)o.o(a,t)&&!o.o(e,t)&&Object.defineProperty(e,t,{enumerable:!0,get:a[t]})},o.f={},o.e=e=>Promise.all(Object.keys(o.f).reduce(((a,t)=>(o.f[t](e,a),a)),[])),o.u=e=>"assets/js/"+({53:"935f2afb",73:"d1095428",269:"2cfedb2c",960:"f57012b0",1021:"4f869f09",1047:"1cd07dfb",1241:"4b4a2981",1489:"5e99f083",1499:"9da79274",1736:"14403852",2087:"b9e8fd80",2191:"ccf76c4f",2490:"1523e34e",2733:"85f8441f",2798:"d92a3c43",3085:"1f391b9e",3299:"6ff4ea8a",4176:"078b4cd8",4392:"e75b870c",4460:"eb02a9a6",4483:"f09f4349",4781:"053840e8",4859:"01497563",5144:"76ada497",5230:"56a89565",5302:"dcd0bb0b",5798:"e6490674",6063:"695c3a47",7060:"62717f00",7323:"f0b082dd",7414:"393be207",7664:"51e4ea00",7678:"3f645037",7776:"7848bf17",7808:"85df453d",7891:"0de68a20",7910:"bd1b2524",7918:"17896441",8010:"5536eb16",8114:"4a9b7d7d",8129:"5712a78f",8217:"e3ece220",8421:"23374ca6",8519:"061587e1",8533:"f6a805cd",8691:"143dbf3b",9290:"9fc7fbc1",9474:"f4bc3419",9514:"1be78505",9534:"83258dd0",9817:"14eb3368"}[e]||e)+"."+{53:"aba92872",73:"0d47130d",269:"aff277fe",960:"873b1b9c",1021:"a780901b",1047:"10d4b610",1241:"299a9d19",1489:"a0c9dbb6",1499:"33fab8f7",1736:"cf7b07b4",2087:"737c9afb",2191:"c62d59af",2490:"cbdbcca6",2666:"e9d8f231",2733:"c9f852bc",2798:"b31ff0ce",3085:"848b92e7",3299:"c5e98112",4176:"111dc502",4392:"96635f55",4460:"b018adb4",4483:"3bd51a26",4781:"f94cc340",4859:"95f764fb",4972:"59a6a108",5144:"29d0d6fb",5230:"f7b4f072",5302:"b17fb064",5798:"e401fc10",6063:"877aae15",7060:"824345ae",7323:"54f2bc2d",7414:"b926d44c",7664:"93ca1b0d",7678:"3ba6e5e7",7776:"864a9f2d",7808:"e0d17945",7891:"9220fcb7",7910:"e9ef6d9e",7918:"7165d1a6",8010:"4a229e90",8114:"8a11dc59",8129:"393b8138",8217:"9cf116b2",8421:"84b83306",8519:"ac1cf6f8",8533:"fc5b9d35",8691:"2214a7b8",9290:"ce069d6d",9474:"c4f0151a",9514:"1063c56f",9534:"f15731f6",9817:"a9c926bf"}[e]+".js",o.miniCssF=e=>{},o.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),o.o=(e,a)=>Object.prototype.hasOwnProperty.call(e,a),d={},r="blockchain-protocols-and-distributed-applications:",o.l=(e,a,t,f)=>{if(d[e])d[e].push(a);else{var c,b;if(void 0!==t)for(var n=document.getElementsByTagName("script"),i=0;i<n.length;i++){var l=n[i];if(l.getAttribute("src")==e||l.getAttribute("data-webpack")==r+t){c=l;break}}c||(b=!0,(c=document.createElement("script")).charset="utf-8",c.timeout=120,o.nc&&c.setAttribute("nonce",o.nc),c.setAttribute("data-webpack",r+t),c.src=e),d[e]=[a];var u=(a,t)=>{c.onerror=c.onload=null,clearTimeout(s);var r=d[e];if(delete d[e],c.parentNode&&c.parentNode.removeChild(c),r&&r.forEach((e=>e(t))),a)return a(t)},s=setTimeout(u.bind(null,void 0,{type:"timeout",target:c}),12e4);c.onerror=u.bind(null,c.onerror),c.onload=u.bind(null,c.onload),b&&document.head.appendChild(c)}},o.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},o.p="/blockchain-protocols-and-distributed-applications/",o.gca=function(e){return e={14403852:"1736",17896441:"7918","935f2afb":"53",d1095428:"73","2cfedb2c":"269",f57012b0:"960","4f869f09":"1021","1cd07dfb":"1047","4b4a2981":"1241","5e99f083":"1489","9da79274":"1499",b9e8fd80:"2087",ccf76c4f:"2191","1523e34e":"2490","85f8441f":"2733",d92a3c43:"2798","1f391b9e":"3085","6ff4ea8a":"3299","078b4cd8":"4176",e75b870c:"4392",eb02a9a6:"4460",f09f4349:"4483","053840e8":"4781","01497563":"4859","76ada497":"5144","56a89565":"5230",dcd0bb0b:"5302",e6490674:"5798","695c3a47":"6063","62717f00":"7060",f0b082dd:"7323","393be207":"7414","51e4ea00":"7664","3f645037":"7678","7848bf17":"7776","85df453d":"7808","0de68a20":"7891",bd1b2524:"7910","5536eb16":"8010","4a9b7d7d":"8114","5712a78f":"8129",e3ece220:"8217","23374ca6":"8421","061587e1":"8519",f6a805cd:"8533","143dbf3b":"8691","9fc7fbc1":"9290",f4bc3419:"9474","1be78505":"9514","83258dd0":"9534","14eb3368":"9817"}[e]||e,o.p+o.u(e)},(()=>{var e={1303:0,532:0};o.f.j=(a,t)=>{var d=o.o(e,a)?e[a]:void 0;if(0!==d)if(d)t.push(d[2]);else if(/^(1303|532)$/.test(a))e[a]=0;else{var r=new Promise(((t,r)=>d=e[a]=[t,r]));t.push(d[2]=r);var f=o.p+o.u(a),c=new Error;o.l(f,(t=>{if(o.o(e,a)&&(0!==(d=e[a])&&(e[a]=void 0),d)){var r=t&&("load"===t.type?"missing":t.type),f=t&&t.target&&t.target.src;c.message="Loading chunk "+a+" failed.\n("+r+": "+f+")",c.name="ChunkLoadError",c.type=r,c.request=f,d[1](c)}}),"chunk-"+a,a)}},o.O.j=a=>0===e[a];var a=(a,t)=>{var d,r,f=t[0],c=t[1],b=t[2],n=0;if(f.some((a=>0!==e[a]))){for(d in c)o.o(c,d)&&(o.m[d]=c[d]);if(b)var i=b(o)}for(a&&a(t);n<f.length;n++)r=f[n],o.o(e,r)&&e[r]&&e[r][0](),e[r]=0;return o.O(i)},t=self.webpackChunkblockchain_protocols_and_distributed_applications=self.webpackChunkblockchain_protocols_and_distributed_applications||[];t.forEach(a.bind(null,0)),t.push=a.bind(null,t.push.bind(t))})()})();