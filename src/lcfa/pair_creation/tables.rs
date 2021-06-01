/// Spacing (in log space) of the sample points in `LN_T_CHI_TABLE`.
pub const DELTA_LN_CHI: f64 = std::f64::consts::LN_10 / 20.0;

/// Table used for calculating the total pair creation rate.
/// Columns of log(chi), log(T(chi)), for sample points
/// in the range 0.01 <= chi <= 100.
pub static LN_T_CHI_TABLE: [[f64; 2]; 81] = [
	[-4.605170185988091e+0, -2.681396200377573e+2],
	[-4.490040931338389e+0, -2.391400775430720e+2],
	[-4.374911676688687e+0, -2.132942556153261e+2],
	[-4.259782422038985e+0, -1.902591957843901e+2],
	[-4.144653167389282e+0, -1.697292366562104e+2],
	[-4.029523912739580e+0, -1.514319579645858e+2],
	[-3.914394658089878e+0, -1.351245657107724e+2],
	[-3.799265403440175e+0, -1.205906704235154e+2],
	[-3.684136148790473e+0, -1.076374157884893e+2],
	[-3.569006894140771e+0, -9.609291954521697e+1],
	[-3.453877639491069e+0, -8.580399269301958e+1],
	[-3.338748384841366e+0, -7.663410674040417e+1],
	[-3.223619130191664e+0, -6.846158202352327e+1],
	[-3.108489875541962e+0, -6.117797305261452e+1],
	[-2.993360620892259e+0, -5.468662945956922e+1],
	[-2.878231366242557e+0, -4.890141344967887e+1],
	[-2.763102111592855e+0, -4.374555673708597e+1],
	[-2.647972856943153e+0, -3.915064179407951e+1],
	[-2.532843602293450e+0, -3.505569389369009e+1],
	[-2.417714347643748e+0, -3.140637189495029e+1],
	[-2.302585092994046e+0, -2.815424703021228e+1],
	[-2.187455838344343e+0, -2.525616012145648e+1],
	[-2.072326583694641e+0, -2.267364869309717e+1],
	[-1.957197329044939e+0, -2.037243637622301e+1],
	[-1.842068074395237e+0, -1.832197782583596e+1],
	[-1.726938819745534e+0, -1.649505310946554e+1],
	[-1.611809565095832e+0, -1.486740618233660e+1],
	[-1.496680310446130e+0, -1.341742264979520e+1],
	[-1.381551055796427e+0, -1.212584253972846e+1],
	[-1.266421801146725e+0, -1.097550427318253e+1],
	[-1.151292546497023e+0, -9.951116436452386e+0],
	[-1.036163291847321e+0, -9.039054328073297e+0],
	[-9.210340371976183e-1, -8.227178584257819e+0],
	[-8.059047825479160e-1, -7.504673480721111e+0],
	[-6.907755278982137e-1, -6.861902771361632e+0],
	[-5.756462732485114e-1, -6.290281158323291e+0],
	[-4.605170185988091e-1, -5.782159696587239e+0],
	[-3.453877639491069e-1, -5.330723622120333e+0],
	[-2.302585092994046e-1, -4.929901258144074e+0],
	[-1.151292546497023e-1, -4.574282801428388e+0],
	[0.000000000000000e+0, -4.259047921583138e+0],
	[1.151292546497023e-1, -3.979901222847447e+0],
	[2.302585092994046e-1, -3.733014721423767e+0],
	[3.453877639491069e-1, -3.514976583381278e+0],
	[4.605170185988091e-1, -3.322745449838084e+0],
	[5.756462732485114e-1, -3.153609748677977e+0],
	[6.907755278982137e-1, -3.005151456506698e+0],
	[8.059047825479160e-1, -2.875213831841356e+0],
	[9.210340371976183e-1, -2.761872691493274e+0],
	[1.036163291847321e+0, -2.663410847495629e+0],
	[1.151292546497023e+0, -2.578295362403297e+0],
	[1.266421801146725e+0, -2.505157316934700e+0],
	[1.381551055796427e+0, -2.442773816242285e+0],
	[1.496680310446130e+0, -2.390051990032361e+0],
	[1.611809565095832e+0, -2.346014767690110e+0],
	[1.726938819745534e+0, -2.309788232834024e+0],
	[1.842068074395237e+0, -2.280590382613709e+0],
	[1.957197329044939e+0, -2.257721135825082e+0],
	[2.072326583694641e+0, -2.240553450765204e+0],
	[2.187455838344343e+0, -2.228525428875320e+0],
	[2.302585092994046e+0, -2.221133293793184e+0],
	[2.417714347643748e+0, -2.217925147603635e+0],
	[2.532843602293450e+0, -2.218495416973374e+0],
	[2.647972856943153e+0, -2.222479911602914e+0],
	[2.763102111592855e+0, -2.229551426135205e+0],
	[2.878231366242557e+0, -2.239415824427002e+0],
	[2.993360620892259e+0, -2.251808552007098e+0],
	[3.108489875541962e+0, -2.266491528699613e+0],
	[3.223619130191664e+0, -2.283250378857702e+0],
	[3.338748384841366e+0, -2.301891961504332e+0],
	[3.453877639491069e+0, -2.322242166976992e+0],
	[3.569006894140771e+0, -2.344143950481519e+0],
	[3.684136148790473e+0, -2.367455576330465e+0],
	[3.799265403440175e+0, -2.392049049622377e+0],
	[3.914394658089878e+0, -2.417808714753949e+0],
	[4.029523912739580e+0, -2.444630002486852e+0],
	[4.144653167389282e+0, -2.472418309350520e+0],
	[4.259782422038985e+0, -2.501087994982919e+0],
	[4.374911676688687e+0, -2.530561484621386e+0],
	[4.490040931338389e+0, -2.560768465379988e+0],
	[4.605170185988091e+0, -2.591645166210501e+0],
];

pub static GL_NODES: [f64; 32] = [
	4.448936583326702e-2,
	2.345261095196185e-1,
	5.768846293018864e-1,
	1.072448753817818e+0,
	1.722408776444645e+0,
	2.528336706425795e+0,
	3.492213273021994e+0,
	4.616456769749767e+0,
	5.903958504174244e+0,
	7.358126733186241e+0,
	8.982940924212596e+0,
	1.078301863253997e+1,
	1.276369798674273e+1,
	1.493113975552256e+1,
	1.729245433671531e+1,
	1.985586094033605e+1,
	2.263088901319677e+1,
	2.562863602245925e+1,
	2.886210181632347e+1,
	3.234662915396474e+1,
	3.610049480575197e+1,
	4.014571977153944e+1,
	4.450920799575494e+1,
	4.922439498730864e+1,
	5.433372133339691e+1,
	5.989250916213402e+1,
	6.597537728793505e+1,
	7.268762809066271e+1,
	8.018744697791352e+1,
	8.873534041789240e+1,
	9.882954286828397e+1,
	1.117513980979377e+2,
];

pub static GL_WEIGHTS: [f64; 32] = [
	1.092183419523850e-1,
	2.104431079388132e-1,
	2.352132296698480e-1,
	1.959033359728810e-1,
	1.299837862860718e-1,
	7.057862386571744e-2,
	3.176091250917507e-2,
	1.191821483483856e-2,
	3.738816294611525e-3,
	9.808033066149551e-4,
	2.148649188013642e-4,
	3.920341967987947e-5,
	5.934541612868633e-6,
	7.416404578667552e-7,
	7.604567879120781e-8,
	6.350602226625807e-9,
	4.281382971040929e-10,
	2.305899491891336e-11,
	9.799379288727094e-13,
	3.237801657729266e-14,
	8.171823443420719e-16,
	1.542133833393823e-17,
	2.119792290163619e-19,
	2.054429673788045e-21,
	1.346982586637395e-23,
	5.661294130397359e-26,
	1.418560545463037e-28,
	1.913375494454224e-31,
	1.192248760098222e-34,
	2.671511219240137e-38,
	1.338616942106256e-42,
	4.510536193898974e-48,
];