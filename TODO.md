# TODO List

## Current Focus: Basic GUI Implementation

### Error Handling
- [ ] Implement `From<core::error::Error>` for `win32::Error`
- [ ] Add `Win32` variant to `core::Error`
- [ ] Consolidate error types between core and win32 modules

### Window API Fixes
- [ ] Fix imports for Windows API types (WNDCLASSA, RegisterClassA)
- [ ] Implement proper string handling for window text (PCSTR conversion)
- [ ] Fix result handling for Windows API calls
- [ ] Remove duplicate window management code between eg/winapi and win32 modules

### Control Implementation
- [ ] Update control constructors to handle HINSTANCE properly:
  - [ ] TreeCtrl
  - [ ] LogCtrl
  - [ ] StatusBar
  - [ ] Toolbar
- [ ] Implement control layout in MainFrame
- [ ] Add window resizing handling

### Code Cleanup
- [ ] Remove unused imports
- [ ] Fix unused variable warnings in plugin registry
- [ ] Fix unused variable warnings in logging code
- [ ] Clean up and document window creation code

### Testing
- [ ] Add basic window creation tests
- [ ] Test window message handling
- [ ] Test control creation and layout
- [ ] Test window cleanup on drop
