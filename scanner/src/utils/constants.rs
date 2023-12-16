pub static SCRIPTS: [&str; 9] = [
  "(default or version or discovery or auth or vuln or external or exploit or malware or safe or intrusive)",
  "and not (broadcast-* or targets-asn or http-robtex-shared-ns or lltd-discovery)",
  "and not (*multicast* or http-icloud-* or hostmap-robtex or http-virustotal)",
  "and not (*dns* or tor-consensus-checker or *domain* or asn-query or http-form-fuzzer)",
  "and not (http-config-backup or mrinfo or http-iis-short-name-brute or http-vuln-cve2013-7091)",
  "and not (http-google-malware or ip-geolocation-map-google or ip-geolocation-map-bing or qscan)",
  "and not (http-useragent-tester or http-mobileversion-checker or *slowloris* or *enum*)",
  "and not (mysql-vuln-cve2012-2122 or http-chrono or eap-info or port-states)",
  "and not (ip-geolocation-map-kml or reverse-index or citrix-brute-xml or http-fetch)",
];
