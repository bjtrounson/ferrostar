package com.stadiamaps.ferrostar

import android.content.Context
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import com.stadiamaps.autocomplete.AutocompleteSearch
import com.stadiamaps.autocomplete.center
import com.stadiamaps.ferrostar.composeui.views.components.gridviews.InnerGridView
import com.stadiamaps.ferrostar.core.LocationProvider
import com.stadiamaps.ferrostar.core.SimulatedLocationProvider
import com.stadiamaps.ferrostar.core.extensions.fromOsrm
import com.stadiamaps.ferrostar.core.toAndroidLocation
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import uniffi.ferrostar.GeographicCoordinate
import uniffi.ferrostar.Route
import uniffi.ferrostar.UserLocation
import uniffi.ferrostar.Waypoint
import uniffi.ferrostar.WaypointKind

@Composable
fun AutocompleteOverlay(
    context: Context,
    modifier: Modifier = Modifier,
    scope: CoroutineScope,
    isNavigating: Boolean,
) {
  if (!isNavigating) {
    InnerGridView(
        modifier = modifier.fillMaxSize().padding(bottom = 16.dp, top = 16.dp),
        topCenter = {
          Button(onClick = { scope.launch {
              var routeBuffer: ByteArray? = null;
              var waypointBuffer: ByteArray? = null;
              try {
                  val stream = context.assets.open("TextToSpeech_Repeating_Voice_Instruction_Bug_Route.json");
                  val size = stream.available();
                  routeBuffer = ByteArray(size);
                  stream.read(routeBuffer);
                  stream.close();
              } catch (e: Exception) {
                  e.printStackTrace();
              }

              try {
                  val stream = context.assets.open("TextToSpeech_Repeating_Voice_Instruction_Bug_Waypoints.json");
                  val size = stream.available();
                  waypointBuffer = ByteArray(size);
                  stream.read(waypointBuffer);
                  stream.close();
              } catch (e: Exception) {
                  e.printStackTrace();
              }

              if (routeBuffer == null || waypointBuffer == null) {
                  return@launch;
              }

              val route = Route.fromOsrm(routeBuffer, waypointBuffer, 6u);

              var locationsRaw: String = "";
              try {
                  val stream = context.assets.open("TextToSpeech_Repeating_Voice_Instruction_Bug_SimulatedLocations.json");
                  val size = stream.available();
                  val locationBuffer = ByteArray(size);
                  stream.read(locationBuffer);
                  stream.close();
                  locationsRaw = locationBuffer.decodeToString();
              } catch (e: Exception) {
                  e.printStackTrace();
              }

              val locations = Gson().fromJson<List<GeographicCoordinate>>(locationsRaw, object : TypeToken<List<GeographicCoordinate>>() {}.type);

              (AppModule.locationProvider as SimulatedLocationProvider).setSimulatedStateFromCoordinates(locations);

              AppModule.ferrostarCore.startNavigation(route = route);
          } }) {
            Text(text = "Start Navigation")
          }
        })
  }
}
