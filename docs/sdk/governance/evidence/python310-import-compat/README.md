# Python3.10 ExecutionUsageValue import compatibility

Declared minimum remains Python>=3.10. Actual3.10.19 public SDK import failed because typing.NotRequired is only in newer stdlib. This patch takes both NotRequired andTypedDict from the already declaredtyping_extensions dependency, asPEP655 requires for Python<3.11. No dependency, minimumversion, field or function behavior changes; ASTafter removingimports is identical.

Three existing/expanded usage tests pass on real3.10 and default3.12. get_type_hints(include_extras=True) retains allthree NotRequired[int|None] annotations; emptymapping differs from suppliednull. CorrectSDKcwdRuff exits0 after fixing importblankline. Raw beforefailure, intermediate attempts, realruntime/annotations andbefore/aftersource hashes remain archived.

Postponedannotations mean __optional_keys__ runtime metadata remains an existing limitation; no claim of fixing that separatecache. No otherfile/owner scope was changed. The coordinatingToollane independently runs its own3.10 native consumers after this import fix; those results are not fabricated here. OfficialPEP655 usage<3.11 is the source for compatibleTypedDict pairing. No package publishing, external commit orsupport-lowerbound change occurred.

Independent review found an intermediate3.10 test environment warning: pytest-asyncio was missing, so pytest did not recognize asyncio_mode. Original3passed/1warning log is retained. Installing the existingconfigured pytest-asyncio plugin in the temporary3.10 environment and repeating the same3tests gives3passed/0warnings. Default3.12 already passed3/0warnings. No warning filtering orsource change was used.
