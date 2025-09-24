use std::collections::HashMap;

// archivo de como manejar mensajes sdp, teniendo en cuenta la estructura del sdp de esta manera:
/* ejemplo:

v=0 – Versión del protocolo.
o=Usuario 2398026505 2307593197 IN IP4 10.20.30.40 – Información sobre el origen de la sesión, incluyendo el identificador del usuario, el identificador de sesión, la versión de sesión, la red y la dirección IP del creador.
s=MyStreamer Audio Session – Nombre de la sesión.
c=IN IP4 10.11.12.13 – Información de conexión, especificando la dirección IP y el tipo de red.
t=0 0 – Tiempo de inicio y finalización de la sesión.
m=audio 15010 RTP/AVP 0 101 – Descripción de los medios, incluyendo el tipo de medio (audio, video, etc.), el puerto, el protocolo de transporte y los formatos de codificación.
a=rtpmap:0 PCMU/8000
a=rtpmap:101 telephone-event/8000

*/
// se podría implementar un enum de errores

/// Represents a complete SDP session description.
#[derive(Debug, PartialEq)]
pub struct SessionDescription {
    pub session_id: String,
    pub session_version: u32,
    pub media_descriptions: Vec<MediaDescription>,
}

/// Represents a media stream (audio or video) within an SDP.
#[derive(Debug, PartialEq)]
pub struct MediaDescription {
    pub media_type: String,
    pub puerto: u16,
    pub protocolo: String,
    pub formato: String,
    pub atributos: HashMap<String, String>,
}

impl SessionDescription {
    /// Crea una nueva oferta SDP con los parámetros dados.
    pub fn nueva_oferta(session_id: &str, session_version: u32, media: MediaDescription) -> Self {
        SessionDescription {
            session_id: session_id.to_string(),
            session_version,
            media_descriptions: vec![media],
        }
    }

    /// Parsea una cadena de texto SDP para llenar la estructura de datos.
    pub fn parsear_cadena(texto_sdp: &str) -> Result<Self, String> {
        let mut lineas = texto_sdp.lineas();
        let mut session_info = SessionDescription {
            session_id: String::new(),
            session_version: 0,
            media_descriptions: Vec::new(),
        };

        while let Some(linea) = lineas.next() {
            let (clave, valor) = parsear_linea(linea)?;
            match clave {
                "o" => parsear_linea_origen(valor, &mut session_info)?,
                "m" => {
                    let media_desc = parsear_medio(valor, &mut lineas)?;
                    session_info.media_descriptions.push(media_desc);
                },
                _ => {} 
            }
        }

        if session_info.session_id.is_empty() {
            return Err("Falta linea =o en SDP".to_string());
        }

        Ok(session_info)
    }

    /// Convierte la estructura de datos SDP a string mediante toString
    // acá iria el to_string
}

// Funciones auxiliares
/// Parsea una línea de texto SDP y la divide en una clave y un valor, ya que una linea SDP tiene el formato clave=valor.
fn parsear_linea(linea: &str) -> Result<(&str, &str), String> {
    let mut partes = linea.splitn(2, '=');
    let clave = match partes.next() {
        Some(clave) => clave,
        None => return Err("Línea SDP inválida: falta '='".to_string()),
    };
    let valor = match partes.next() {
        Some(valor) => valor,
        None => return Err("Línea SDP inválida: falta el valor".to_string()),
    };
    Ok((clave, valor))
}

/// Parsea la línea de origen 'o=' y actualiza la información de la sesión.
//La línea "o =" contiene el id de la sesión y la versión de la sesión.
// un ejemplo: o=- 123456789 1 IN IP4 127.0.0.1
fn parsear_linea_origen(valor: &str, info_sesion: &mut SessionDescription) -> Result<(), String> {
    let partes: Vec<&str> = valor.split_whitespace().collect();
    if partes.len() < 2 {
        return Err("Línea o= invalida".to_string());
    }

    info_sesion.session_id = partes[1].to_string();

    match partes[2].parse::<u32>() {
        Ok(version) => {
            info_sesion.session_version = version;
            Ok(())
        },
        Err(_) => Err("Línea o= inválida: la versión no es un número".to_string()),
    }
}

/// Parsea la línea de medios "m="" y sus atributos "a=""
// Consume las líneas siguientes al "m="" hasta encontrar otra línea de medios o el final del SDP
// hay que implementarlo, que llene los datos del MediaDescription y devuelva MEdiaDescription
/*

fn parsear_medio(valor: &str, lineas: &mut std::str::Lines) -> Result<MediaDescription, String> {
    let partes_m: Vec<&str> = valor.split_whitespace().collect();
    if partes_m.len() < 4 {
        return Err("Línea 'm=' inválida: malformada".to_string());
    }
    let tipo_medio = partes_m[0].to_string();

    Ok(MediaDescription {tipo_medio, puerto, protocolo, formato,atributos})
}

*/