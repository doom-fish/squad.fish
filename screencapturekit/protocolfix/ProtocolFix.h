
#import <Foundation/Foundation.h>
#import <ScreenCaptureKit/SCStream.h>

@interface ProtocolFix: NSObject<SCStreamOutput, SCStreamDelegate> {}
@end

