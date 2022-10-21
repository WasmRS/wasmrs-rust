import { RustBasic } from "@apexlang/codegen/rust";
var RequestType;
(function (RequestType) {
    RequestType[RequestType["RequestResponse"] = 0] = "RequestResponse";
    RequestType[RequestType["RequestChannel"] = 1] = "RequestChannel";
    RequestType[RequestType["RequestStream"] = 2] = "RequestStream";
    RequestType[RequestType["FireAndForget"] = 3] = "FireAndForget";
})(RequestType || (RequestType = {}));
export class DefaultVisitor extends RustBasic {
    constructor() {
        super(...arguments);
        this.namespace = "";
    }
    visitContextBefore(context) {
        /*
          If a "header" option exists in the configuration, add it to the
          generated output. Useful for license or contact information.
        */
        if (context.config.header) {
            if (Array.isArray(context.config.header)) {
                // If it's an array, join each line with a newline.
                this.write(context.config.header.join("\n"));
            }
            else {
                // Otherwise add it directly.
                this.write(context.config.header);
            }
        }
    }
    visitContextAfter(context) {
        /*
          If a "footer" option exists in the configuration, add it to the
          generated output.
        */
        if (context.config.footer) {
            if (Array.isArray(context.config.footer)) {
                this.write(context.config.footer.join("\n"));
            }
            else {
                this.write(context.config.footer);
            }
        }
    }
    visitNamespace(context) {
        const { namespace } = context;
        this.namespace = namespace.name;
    }
    visitFunction(context) {
        const func = context.operation;
        const operation = {
            name: func.name,
            namespace: this.namespace,
            type: RequestType.RequestChannel,
            inputs: {},
            outputs: {},
        };
        // this.write(convertOperation(context.operation));
    }
}
function convertOperation(operation) {
    // let opName = utils.rustifyCaps(operation.name);
    // let inputs = Object.entries(operation.inputs).map((name, type) => {});
    return "";
    //   return `
    // pub mod ${opName} {
    //   pub struct Inputs {
    //     ${""}
    //   }
    //   pub struct Outputs {
    //     ${""}
    //   }
    //   pub struct Component {}
    //   impl RequestChannel for Component {
    //     fn request_channel_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    //       ${}
    //     }
    //   }
    // }
    // pub(crate) type GEN_RC_INPUTS = FluxReceiver<String, PayloadError>;
    // pub(crate) type GEN_RC_OUTPUTS = Flux<String, PayloadError>;
    // pub(crate) struct GEN_RC {}
    // impl RequestChannel for GEN_RC {
    //     fn request_channel_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    //         // generated
    //         let (inputs_tx, inputs_rx) = Flux::<String, PayloadError>::new_parts();
    //         spawn(async move {
    //             while let Ok(Some(Ok(payload))) = input.recv().await {
    //                 inputs_tx.send_result(deserialize(&payload.data).map_err(|e| e.into()));
    //             }
    //         });
    //         let (real_out_tx, real_out_rx) = Flux::new_parts();
    //         let (outputs_tx, mut outputs_rx) = Flux::new_parts();
    //         spawn(async move {
    //             while let Some(result) = outputs_rx.next().await {
    //                 match result {
    //                     Ok(payload) => match serialize(&payload) {
    //                         Ok(bytes) => {
    //                             real_out_tx.send(Payload::new_optional(None, Some(Bytes::from(bytes))));
    //                         }
    //                         Err(e) => {
    //                             real_out_tx.error(PayloadError::application_error(e.to_string()));
    //                         }
    //                     },
    //                     Err(err) => {
    //                         real_out_tx.error(err);
    //                     }
    //                 }
    //             }
    //         });
    //         spawn(async move {
    //             let _result = Self {}.task(inputs_rx, outputs_tx).await;
    //         });
    //         Ok(real_out_rx)
    //     }
    // }
    //   `;
}
//# sourceMappingURL=default-visitor%20copy.js.map