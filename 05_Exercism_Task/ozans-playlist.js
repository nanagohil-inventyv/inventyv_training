// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Removes duplicate tracks from a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} new playlist with unique entries
 */
export function removeDuplicates(playlist) {
     const uniquePlayList = new Set(playlist)
    return Array.from(uniquePlayList);
}

/**
 * Checks whether a playlist includes a track.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {boolean} whether the track is in the playlist
 */
export function hasTrack(playlist, track) {
     const playListSet = new Set(playlist);
     return playListSet.has(track);
}

/**
 * Adds a track to a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function addTrack(playlist, track) {
      const uniquePlayList = new Set(playlist)
      uniquePlayList.add(track);
      return Array.from(uniquePlayList);
}

/**
 * Deletes a track from a playlist.
 *
 * @param {string[]} playlist
 * @param {string} track
 * @returns {string[]} new playlist
 */
export function deleteTrack(playlist, track) {
      const uniquePlayList = new Set(playlist)
      uniquePlayList.delete(track);
      return Array.from(uniquePlayList);
}

/**
 * Lists the unique artists in a playlist.
 *
 * @param {string[]} playlist
 * @returns {string[]} list of artists
 */
export function listArtists(playlist) {
   const uniqueArtists = new Set();
   for(let song of playlist){
     const [_ ,artist] = song.split('- ')
     uniqueArtists.add(artist);
   }
  return Array.from(uniqueArtists);
}
