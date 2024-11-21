export function getRealText(text: string) {
	switch (text) {
		case "pótlék_délelőtti":
			return "Délelőtti pótlékok";
		case "pótlék_éjszakai":
			return "Éjszakai pótlékok";
		case "számla":
			return "Számlák";
		case "leintés":
			return "Leintések";
	}
}

export function getAlterText(text: string) {
	switch (text) {
		case "pótlék_délelőtti":
			return "potlek_de";
		case "pótlék_éjszakai":
			return "potlek_ej";
		case "számla":
			return "szamla";
		case "leintés":
			return "leintes";
	}
}

const Reeler = {
	leintesek: ["leintés", "Leintéseid", "Leintés", "Leintések", "leintésének"],
	potlekok: ["pótlék", "Pótlékaid", "Pótlék", "Pótlékok", "pótlékjának"],
	szamlak: ["számla", "Számláid", "Számla", "Számlák", "számlájának"],
};

export const Reeler_keys = Object.keys(Reeler);
export const Reeler_vals = Object.values(Reeler);