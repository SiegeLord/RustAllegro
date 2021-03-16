(function() {var implementors = {};
implementors["allegro"] = [{"text":"impl Sync for BitmapFlags","synthetic":true,"types":[]},{"text":"impl Sync for Color","synthetic":true,"types":[]},{"text":"impl Sync for PixelFormat","synthetic":true,"types":[]},{"text":"impl&lt;'l&gt; !Sync for ConfigSection&lt;'l&gt;","synthetic":true,"types":[]},{"text":"impl&lt;'l&gt; !Sync for ConfigEntry&lt;'l&gt;","synthetic":true,"types":[]},{"text":"impl Sync for BitmapDrawingFlags","synthetic":true,"types":[]},{"text":"impl Sync for BlendMode","synthetic":true,"types":[]},{"text":"impl Sync for BlendOperation","synthetic":true,"types":[]},{"text":"impl Sync for MonitorInfo","synthetic":true,"types":[]},{"text":"impl Sync for DepthFunction","synthetic":true,"types":[]},{"text":"impl Sync for Core","synthetic":true,"types":[]},{"text":"impl Sync for DisplayFlags","synthetic":true,"types":[]},{"text":"impl Sync for DisplayOption","synthetic":true,"types":[]},{"text":"impl Sync for DisplayOptionImportance","synthetic":true,"types":[]},{"text":"impl Sync for DisplayOrientation","synthetic":true,"types":[]},{"text":"impl !Sync for EventQueue","synthetic":true,"types":[]},{"text":"impl&lt;'l&gt; !Sync for EventSource&lt;'l&gt;","synthetic":true,"types":[]},{"text":"impl Sync for UserEventSource","synthetic":true,"types":[]},{"text":"impl !Sync for Event","synthetic":true,"types":[]},{"text":"impl Sync for StickFlags","synthetic":true,"types":[]},{"text":"impl !Sync for Joystick","synthetic":true,"types":[]},{"text":"impl Sync for KeyCode","synthetic":true,"types":[]},{"text":"impl Sync for KeyModifier","synthetic":true,"types":[]},{"text":"impl Sync for ShaderPlatform","synthetic":true,"types":[]},{"text":"impl Sync for ShaderType","synthetic":true,"types":[]},{"text":"impl !Sync for Timer","synthetic":true,"types":[]},{"text":"impl Sync for Transform","synthetic":true,"types":[]},{"text":"impl Sync for Bitmap","synthetic":false,"types":[]},{"text":"impl Sync for SubBitmap","synthetic":false,"types":[]},{"text":"impl Sync for Config","synthetic":false,"types":[]},{"text":"impl Sync for Display","synthetic":false,"types":[]},{"text":"impl Sync for Shader","synthetic":false,"types":[]}];
implementors["allegro_acodec"] = [{"text":"impl Sync for AcodecAddon","synthetic":true,"types":[]}];
implementors["allegro_audio"] = [{"text":"impl Sync for AudioAddon","synthetic":true,"types":[]},{"text":"impl Sync for AudioDepth","synthetic":true,"types":[]},{"text":"impl Sync for ChannelConf","synthetic":true,"types":[]},{"text":"impl Sync for Playmode","synthetic":true,"types":[]},{"text":"impl Sync for MixerQuality","synthetic":true,"types":[]},{"text":"impl Sync for Mixer","synthetic":false,"types":[]},{"text":"impl Sync for Sample","synthetic":false,"types":[]},{"text":"impl Sync for SampleInstance","synthetic":false,"types":[]},{"text":"impl Sync for Sink","synthetic":false,"types":[]},{"text":"impl Sync for AudioStream","synthetic":false,"types":[]}];
implementors["allegro_audio_sys"] = [{"text":"impl Sync for ALLEGRO_AUDIO_DEPTH","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_CHANNEL_CONF","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_PLAYMODE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_MIXER_QUALITY","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_SAMPLE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_SAMPLE_ID","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_SAMPLE_INSTANCE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_AUDIO_STREAM","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_MIXER","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_VOICE","synthetic":true,"types":[]}];
implementors["allegro_dialog"] = [{"text":"impl Sync for MessageBoxFlags","synthetic":true,"types":[]},{"text":"impl Sync for MessageBoxResult","synthetic":true,"types":[]},{"text":"impl Sync for DialogAddon","synthetic":true,"types":[]}];
implementors["allegro_dialog_sys"] = [{"text":"impl Sync for ALLEGRO_FILECHOOSER","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_TEXTLOG","synthetic":true,"types":[]}];
implementors["allegro_font"] = [{"text":"impl Sync for FontAddon","synthetic":true,"types":[]},{"text":"impl Sync for FontAlign","synthetic":true,"types":[]},{"text":"impl Sync for Font","synthetic":false,"types":[]}];
implementors["allegro_font_sys"] = [{"text":"impl !Sync for ALLEGRO_FONT","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_FONT_VTABLE","synthetic":true,"types":[]}];
implementors["allegro_image"] = [{"text":"impl Sync for ImageAddon","synthetic":true,"types":[]}];
implementors["allegro_primitives"] = [{"text":"impl Sync for PrimType","synthetic":true,"types":[]},{"text":"impl Sync for LineJoinType","synthetic":true,"types":[]},{"text":"impl Sync for LineCapType","synthetic":true,"types":[]},{"text":"impl Sync for PrimitivesAddon","synthetic":true,"types":[]},{"text":"impl Sync for Vertex","synthetic":true,"types":[]},{"text":"impl Sync for VertexDeclBuilder","synthetic":true,"types":[]},{"text":"impl Sync for VertexAttrStorage","synthetic":true,"types":[]},{"text":"impl !Sync for VertexDecl","synthetic":true,"types":[]}];
implementors["allegro_primitives_sys"] = [{"text":"impl Sync for ALLEGRO_VERTEX_ELEMENT","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_VERTEX_DECL","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_VERTEX","synthetic":true,"types":[]}];
implementors["allegro_sys"] = [{"text":"impl Sync for ALLEGRO_TIMEOUT","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_BITMAP","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_LOCKED_REGION","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_COLOR","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_CONFIG_SECTION","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_CONFIG_ENTRY","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_CONFIG","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_DISPLAY","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_EVENT_SOURCE","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_ANY_EVENT","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_DISPLAY_EVENT","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_JOYSTICK_EVENT","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_KEYBOARD_EVENT","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_MOUSE_EVENT","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_TIMER_EVENT","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_USER_EVENT","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_EVENT","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_EVENT_QUEUE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_FILE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_FILE_INTERFACE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_SEEK","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_JOYSTICK","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_JOYSTICK_STATE","synthetic":true,"types":[]},{"text":"impl Sync for Stick","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_KEYBOARD","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_KEYBOARD_STATE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_MONITOR_INFO","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_MOUSE","synthetic":true,"types":[]},{"text":"impl !Sync for ALLEGRO_MOUSE_STATE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_PATH","synthetic":true,"types":[]},{"text":"impl !Sync for __al_tagbstring","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_RENDER_STATE","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_RENDER_FUNCTION","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_WRITE_MASK_FLAGS","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_SHADER","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_SYSTEM","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_TIMER","synthetic":true,"types":[]},{"text":"impl Sync for ALLEGRO_TRANSFORM","synthetic":true,"types":[]}];
implementors["allegro_ttf"] = [{"text":"impl Sync for TtfFlags","synthetic":true,"types":[]},{"text":"impl Sync for TtfAddon","synthetic":true,"types":[]}];
implementors["libc"] = [{"text":"impl Sync for statvfs","synthetic":true,"types":[]},{"text":"impl Sync for max_align_t","synthetic":true,"types":[]},{"text":"impl Sync for sigaction","synthetic":true,"types":[]},{"text":"impl Sync for statfs","synthetic":true,"types":[]},{"text":"impl Sync for flock","synthetic":true,"types":[]},{"text":"impl Sync for flock64","synthetic":true,"types":[]},{"text":"impl Sync for siginfo_t","synthetic":true,"types":[]},{"text":"impl !Sync for stack_t","synthetic":true,"types":[]},{"text":"impl Sync for stat","synthetic":true,"types":[]},{"text":"impl Sync for stat64","synthetic":true,"types":[]},{"text":"impl Sync for statfs64","synthetic":true,"types":[]},{"text":"impl Sync for statvfs64","synthetic":true,"types":[]},{"text":"impl Sync for pthread_attr_t","synthetic":true,"types":[]},{"text":"impl Sync for _libc_fpxreg","synthetic":true,"types":[]},{"text":"impl Sync for _libc_xmmreg","synthetic":true,"types":[]},{"text":"impl Sync for _libc_fpstate","synthetic":true,"types":[]},{"text":"impl Sync for user_regs_struct","synthetic":true,"types":[]},{"text":"impl !Sync for user","synthetic":true,"types":[]},{"text":"impl !Sync for mcontext_t","synthetic":true,"types":[]},{"text":"impl Sync for ipc_perm","synthetic":true,"types":[]},{"text":"impl Sync for shmid_ds","synthetic":true,"types":[]},{"text":"impl Sync for termios2","synthetic":true,"types":[]},{"text":"impl Sync for ip_mreqn","synthetic":true,"types":[]},{"text":"impl Sync for user_fpregs_struct","synthetic":true,"types":[]},{"text":"impl !Sync for ucontext_t","synthetic":true,"types":[]},{"text":"impl Sync for sigset_t","synthetic":true,"types":[]},{"text":"impl Sync for sysinfo","synthetic":true,"types":[]},{"text":"impl Sync for msqid_ds","synthetic":true,"types":[]},{"text":"impl Sync for sem_t","synthetic":true,"types":[]},{"text":"impl Sync for statx","synthetic":true,"types":[]},{"text":"impl Sync for statx_timestamp","synthetic":true,"types":[]},{"text":"impl !Sync for aiocb","synthetic":true,"types":[]},{"text":"impl Sync for __exit_status","synthetic":true,"types":[]},{"text":"impl Sync for __timeval","synthetic":true,"types":[]},{"text":"impl !Sync for glob64_t","synthetic":true,"types":[]},{"text":"impl !Sync for msghdr","synthetic":true,"types":[]},{"text":"impl Sync for cmsghdr","synthetic":true,"types":[]},{"text":"impl Sync for termios","synthetic":true,"types":[]},{"text":"impl Sync for mallinfo","synthetic":true,"types":[]},{"text":"impl Sync for nlmsghdr","synthetic":true,"types":[]},{"text":"impl Sync for nlmsgerr","synthetic":true,"types":[]},{"text":"impl Sync for nl_pktinfo","synthetic":true,"types":[]},{"text":"impl Sync for nl_mmap_req","synthetic":true,"types":[]},{"text":"impl Sync for nl_mmap_hdr","synthetic":true,"types":[]},{"text":"impl Sync for nlattr","synthetic":true,"types":[]},{"text":"impl !Sync for rtentry","synthetic":true,"types":[]},{"text":"impl Sync for timex","synthetic":true,"types":[]},{"text":"impl Sync for ntptimeval","synthetic":true,"types":[]},{"text":"impl !Sync for regex_t","synthetic":true,"types":[]},{"text":"impl Sync for Elf64_Chdr","synthetic":true,"types":[]},{"text":"impl Sync for Elf32_Chdr","synthetic":true,"types":[]},{"text":"impl Sync for utmpx","synthetic":true,"types":[]},{"text":"impl Sync for fpos64_t","synthetic":true,"types":[]},{"text":"impl Sync for rlimit64","synthetic":true,"types":[]},{"text":"impl !Sync for glob_t","synthetic":true,"types":[]},{"text":"impl !Sync for passwd","synthetic":true,"types":[]},{"text":"impl !Sync for spwd","synthetic":true,"types":[]},{"text":"impl Sync for dqblk","synthetic":true,"types":[]},{"text":"impl Sync for signalfd_siginfo","synthetic":true,"types":[]},{"text":"impl Sync for itimerspec","synthetic":true,"types":[]},{"text":"impl Sync for fsid_t","synthetic":true,"types":[]},{"text":"impl Sync for packet_mreq","synthetic":true,"types":[]},{"text":"impl Sync for cpu_set_t","synthetic":true,"types":[]},{"text":"impl !Sync for if_nameindex","synthetic":true,"types":[]},{"text":"impl Sync for msginfo","synthetic":true,"types":[]},{"text":"impl Sync for sembuf","synthetic":true,"types":[]},{"text":"impl Sync for input_event","synthetic":true,"types":[]},{"text":"impl Sync for input_id","synthetic":true,"types":[]},{"text":"impl Sync for input_absinfo","synthetic":true,"types":[]},{"text":"impl Sync for input_keymap_entry","synthetic":true,"types":[]},{"text":"impl Sync for input_mask","synthetic":true,"types":[]},{"text":"impl Sync for ff_replay","synthetic":true,"types":[]},{"text":"impl Sync for ff_trigger","synthetic":true,"types":[]},{"text":"impl Sync for ff_envelope","synthetic":true,"types":[]},{"text":"impl Sync for ff_constant_effect","synthetic":true,"types":[]},{"text":"impl Sync for ff_ramp_effect","synthetic":true,"types":[]},{"text":"impl Sync for ff_condition_effect","synthetic":true,"types":[]},{"text":"impl !Sync for ff_periodic_effect","synthetic":true,"types":[]},{"text":"impl Sync for ff_rumble_effect","synthetic":true,"types":[]},{"text":"impl Sync for ff_effect","synthetic":true,"types":[]},{"text":"impl Sync for uinput_ff_upload","synthetic":true,"types":[]},{"text":"impl Sync for uinput_ff_erase","synthetic":true,"types":[]},{"text":"impl Sync for uinput_abs_setup","synthetic":true,"types":[]},{"text":"impl !Sync for dl_phdr_info","synthetic":true,"types":[]},{"text":"impl Sync for Elf32_Ehdr","synthetic":true,"types":[]},{"text":"impl Sync for Elf64_Ehdr","synthetic":true,"types":[]},{"text":"impl Sync for Elf32_Sym","synthetic":true,"types":[]},{"text":"impl Sync for Elf64_Sym","synthetic":true,"types":[]},{"text":"impl Sync for Elf32_Phdr","synthetic":true,"types":[]},{"text":"impl Sync for Elf64_Phdr","synthetic":true,"types":[]},{"text":"impl Sync for Elf32_Shdr","synthetic":true,"types":[]},{"text":"impl Sync for Elf64_Shdr","synthetic":true,"types":[]},{"text":"impl Sync for ucred","synthetic":true,"types":[]},{"text":"impl !Sync for mntent","synthetic":true,"types":[]},{"text":"impl !Sync for posix_spawn_file_actions_t","synthetic":true,"types":[]},{"text":"impl Sync for posix_spawnattr_t","synthetic":true,"types":[]},{"text":"impl Sync for genlmsghdr","synthetic":true,"types":[]},{"text":"impl Sync for in6_pktinfo","synthetic":true,"types":[]},{"text":"impl Sync for arpd_request","synthetic":true,"types":[]},{"text":"impl Sync for inotify_event","synthetic":true,"types":[]},{"text":"impl Sync for fanotify_response","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_vm","synthetic":true,"types":[]},{"text":"impl Sync for regmatch_t","synthetic":true,"types":[]},{"text":"impl Sync for sock_extended_err","synthetic":true,"types":[]},{"text":"impl Sync for __c_anonymous_sockaddr_can_tp","synthetic":true,"types":[]},{"text":"impl Sync for __c_anonymous_sockaddr_can_j1939","synthetic":true,"types":[]},{"text":"impl Sync for can_filter","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_nl","synthetic":true,"types":[]},{"text":"impl Sync for dirent","synthetic":true,"types":[]},{"text":"impl Sync for dirent64","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_alg","synthetic":true,"types":[]},{"text":"impl Sync for uinput_setup","synthetic":true,"types":[]},{"text":"impl Sync for uinput_user_dev","synthetic":true,"types":[]},{"text":"impl Sync for af_alg_iv","synthetic":true,"types":[]},{"text":"impl Sync for mq_attr","synthetic":true,"types":[]},{"text":"impl Sync for __c_anonymous_sockaddr_can_can_addr","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_can","synthetic":true,"types":[]},{"text":"impl Sync for pthread_mutexattr_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_rwlockattr_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_condattr_t","synthetic":true,"types":[]},{"text":"impl Sync for fanotify_event_metadata","synthetic":true,"types":[]},{"text":"impl Sync for pthread_cond_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_mutex_t","synthetic":true,"types":[]},{"text":"impl Sync for pthread_rwlock_t","synthetic":true,"types":[]},{"text":"impl Sync for can_frame","synthetic":true,"types":[]},{"text":"impl Sync for canfd_frame","synthetic":true,"types":[]},{"text":"impl Sync for timezone","synthetic":true,"types":[]},{"text":"impl Sync for in_addr","synthetic":true,"types":[]},{"text":"impl Sync for ip_mreq","synthetic":true,"types":[]},{"text":"impl Sync for ip_mreq_source","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_in","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_in6","synthetic":true,"types":[]},{"text":"impl !Sync for addrinfo","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_ll","synthetic":true,"types":[]},{"text":"impl Sync for fd_set","synthetic":true,"types":[]},{"text":"impl !Sync for tm","synthetic":true,"types":[]},{"text":"impl Sync for sched_param","synthetic":true,"types":[]},{"text":"impl !Sync for Dl_info","synthetic":true,"types":[]},{"text":"impl !Sync for lconv","synthetic":true,"types":[]},{"text":"impl Sync for in_pktinfo","synthetic":true,"types":[]},{"text":"impl !Sync for ifaddrs","synthetic":true,"types":[]},{"text":"impl Sync for in6_rtmsg","synthetic":true,"types":[]},{"text":"impl Sync for arpreq","synthetic":true,"types":[]},{"text":"impl Sync for arpreq_old","synthetic":true,"types":[]},{"text":"impl Sync for arphdr","synthetic":true,"types":[]},{"text":"impl !Sync for mmsghdr","synthetic":true,"types":[]},{"text":"impl Sync for epoll_event","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_un","synthetic":true,"types":[]},{"text":"impl Sync for sockaddr_storage","synthetic":true,"types":[]},{"text":"impl Sync for utsname","synthetic":true,"types":[]},{"text":"impl !Sync for sigevent","synthetic":true,"types":[]},{"text":"impl Sync for in6_addr","synthetic":true,"types":[]},{"text":"impl Sync for DIR","synthetic":true,"types":[]},{"text":"impl !Sync for group","synthetic":true,"types":[]},{"text":"impl Sync for utimbuf","synthetic":true,"types":[]},{"text":"impl Sync for timeval","synthetic":true,"types":[]},{"text":"impl Sync for timespec","synthetic":true,"types":[]},{"text":"impl Sync for rlimit","synthetic":true,"types":[]},{"text":"impl Sync for rusage","synthetic":true,"types":[]},{"text":"impl Sync for ipv6_mreq","synthetic":true,"types":[]},{"text":"impl !Sync for hostent","synthetic":true,"types":[]},{"text":"impl !Sync for iovec","synthetic":true,"types":[]},{"text":"impl Sync for pollfd","synthetic":true,"types":[]},{"text":"impl Sync for winsize","synthetic":true,"types":[]},{"text":"impl Sync for linger","synthetic":true,"types":[]},{"text":"impl !Sync for sigval","synthetic":true,"types":[]},{"text":"impl Sync for itimerval","synthetic":true,"types":[]},{"text":"impl Sync for tms","synthetic":true,"types":[]},{"text":"impl !Sync for servent","synthetic":true,"types":[]},{"text":"impl !Sync for protoent","synthetic":true,"types":[]},{"text":"impl Sync for FILE","synthetic":true,"types":[]},{"text":"impl Sync for fpos_t","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()