# Downloading Yang-Models from the Switch

The provided Pythonscript enables you to download all advertised models in the servers <hello> Message.
Unfortunately the used B&R-Switch doesnt announce all used Yang-Models properly so the rest has to be downloaded manually.

## using the provided python script
All Libraries can be installed via pip and the script can be run without any arguments.

## Manual Netconf requests
For Netconf 1.0 Messages are send, when ending with "]]>]]>".
For Netconf 1.1 the bytesize has to be provided which is not as easy to do manually, so we just use 1.0.

### \<hello> response
```xml
<?xml version="1.0" encoding="UTF-8"?>
<hello xmlns="urn:ietf:params:xml:ns:netconf:base:1.0">
    <capabilities>
        <capability>urn:ietf:params:netconf:base:1.0</capability>
    </capabilities>
</hello>
```

### \<get-schema> for downloading a specific yang-model
```xml
<?xml version="1.0" encoding="utf-8"?>
<rpc xmlns="urn:ietf:params:xml:ns:netconf:base:1.0" message-id="1">
  <get-schema xmlns="urn:ietf:params:xml:ns:yang:ietf-netconf-monitoring">
    <identifier>ieee802-types</identifier>
    <format>yang</format>
  </get-schema>
</rpc>
```

### \<get> all yang-models present on the switch
```xml
<?xml version="1.0" encoding="utf-8"?>
<rpc xmlns="urn:ietf:params:xml:ns:netconf:base:1.0" message-id="2">
  <get>
    <filter type="subtree">
      <modules-state xmlns="urn:ietf:params:xml:ns:yang:ietf-yang-library">
        <module />
      </modules-state>
    </filter>
  </get>
</rpc>
```
