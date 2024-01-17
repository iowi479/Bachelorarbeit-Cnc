import re
from ncclient import manager

# This script is for downloading present Yang-models from a switch
# This is only tested on the specifc B&R Switch used in the corrseponding bachelor-thesis

with manager.connect(
    host='0.0.0.0',
    port=830,
    username='user',
    password='password',
    hostkey_verify=False
) as m:
    capabilities = []

    for capability in m.server_capabilities:
         capabilities.append(capability)

    capabilities = sorted(capabilities)

    modules = []
    for capability in capabilities:
        # extract module-name from capability-data
        supported_model = re.search('module=(.*)&', capability)
        if supported_model is not None:
            print("Supported Model: %s" % supported_model.group(1))
            modules.append(supported_model.groups(0)[0])

    for model in modules:
        print("Requesting: %s" % model)
        # model includes revision tag
        if "&" in model:
            model = model.split("&")[0]

        schema = m.get_schema(model)

        with open("./{}.yang".format(model), 'w') as f:
            f.write(schema.data)