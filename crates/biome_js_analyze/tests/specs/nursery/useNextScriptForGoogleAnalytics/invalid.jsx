/* should generate diagnostics */
<script async src="https://www.googletagmanager.com/gtag/js?id=G-XYZ" />;

<script async src={`https://www.googletagmanager.com/gtag/js?id=${GA_TRACKING_ID}`} />;

<script async src={"https://www.googletagmanager.com/gtm.js?id=GTM-XYZ"} />;

<script async src='https://www.google-analytics.com/analytics.js'></script>;

<script
	dangerouslySetInnerHTML={{
		__html: `
			(function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
			(i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),
			m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
			})(window,document,'script','https://www.google-analytics.com/analytics.js','ga');
			ga('create', 'UA-XXXXX-Y', 'auto');
			ga('send', 'pageview');
		`,
	}}
/>;

<script
	dangerouslySetInnerHTML={{
		__html: `
			(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
			new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
			j=d.createElement(s),dl=l!='dataLayer'?'&l='+l:'';j.async=true;j.src=
			'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
			})(window,document,'script','dataLayer','${GTM_ID}');
		`,
	}}
/>;

<script dangerouslySetInnerHTML={{ __html: "j.src='https://www.googletagmanager.com/gtm.js?id=GTM-XYZ'" }} />;
